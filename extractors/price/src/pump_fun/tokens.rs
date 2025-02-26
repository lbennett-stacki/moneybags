use super::program::{program::get_pump_fun_program_address, signatures::TransactionSignature};
use crate::{
    cpi::cpi::CpiLog,
    crawl_status::{
        queries::has_crawled_signature,
        table::{CrawlStatusOperation, CrawlStatusRowIdentifier},
    },
    dragonfly::client::dragonfly_client,
    instructions::instruction::Instruction,
    pump_fun::instructions::PumpFunInstruction,
    raydium::amm::get_raydium_amm_program_address,
    rpc::{clients::get_rpc_nodes_count, pool::RpcPoolManager},
    system::program::get_system_program_address,
    trades::table::{TradeDirection, TradeRow},
    transactions::{
        config::TRANSACTION_CONFIG, parse::parse_transaction_with_logs,
        status::is_failed_transaction,
    },
    utils::log::log_time,
};
use crossbeam::channel::{Receiver, Sender};
use dashmap::DashSet;
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use solana_transaction_status::{EncodedTransaction, UiMessage};
use std::{str::FromStr, sync::Arc, thread};
use time::OffsetDateTime;

const CONCURRENCY: usize = 1;

pub type MintAddress = Pubkey;
pub type BondingCurveAddress = Pubkey;
pub type PumpFunToken = (MintAddress, BondingCurveAddress);

pub fn get_pump_fun_tokens_from_pump_fun_program_signatures(
    pump_fun_tokens_tx: &Sender<PumpFunToken>,
    trades_tx: &Sender<TradeRow>,
    pump_fun_program_signatures_rx: &Receiver<TransactionSignature>,
    crawl_status_tx: &Sender<CrawlStatusOperation>,
    rpc_pool_manager: &RpcPoolManager,
) -> Vec<thread::JoinHandle<()>> {
    let concurrency = get_rpc_nodes_count();
    let mut handles = Vec::with_capacity(concurrency);

    let pump_fun_program_address = get_pump_fun_program_address();
    let system_program_address = get_system_program_address();

    let raydium_amm_program_address = get_raydium_amm_program_address();

    let already_seen_mint_addresses = Arc::new(DashSet::new());

    for thread_index in 0..CONCURRENCY {
        let log_tag = format!(
            "     {} pump fun coins from transactions #{} | ",
            log_time(),
            thread_index
        );

        let pump_fun_tokens_tx = pump_fun_tokens_tx.clone();
        let rpc_pool_manager = rpc_pool_manager.clone();
        let pump_fun_signatures_rx = pump_fun_program_signatures_rx.clone();
        let trades_tx = trades_tx.clone();
        let crawl_status_tx = crawl_status_tx.clone();
        let already_seen_mint_addresses = already_seen_mint_addresses.clone();
        let dragonfly_client = dragonfly_client();

        let handle = thread::spawn(move || {
            while let Ok(pump_fun_program_signature) = pump_fun_signatures_rx.recv() {
                println!("{} Crawling pump fun transaction", log_tag);
                if let Ok(has_crawled) =
                    has_crawled_signature(&dragonfly_client, &pump_fun_program_signature)
                {
                    if has_crawled {
                        println!(
                            "{} Pump fun program signature already crawled. Skipping",
                            log_tag
                        );
                        continue;
                    }
                }

                println!(
                    "\n     --------------------------------------------\n\n{} Getting mint addresses for {}",
                    log_tag, pump_fun_program_signature
                );
                let sig = Signature::from_str(&pump_fun_program_signature).unwrap();
                let tx = rpc_pool_manager.execute(
                    |client| client.get_transaction_with_config(&sig, TRANSACTION_CONFIG),
                    Some(thread_index as u64),
                );

                match tx {
                    Err(error) => {
                        println!(
                            "{} Error getting mint address.\n{:?}\nSkipping",
                            log_tag, error
                        );
                        crawl_status_tx
                            .send(CrawlStatusOperation::MarkAsFailed(
                                CrawlStatusRowIdentifier {
                                    account_address: pump_fun_program_address.to_string(),
                                    transaction_signature: pump_fun_program_signature.clone(),
                                },
                                error.to_string(),
                            ))
                            .unwrap();
                        continue;
                    }
                    Ok(tx) => {
                        if is_failed_transaction(&tx) {
                            println!("{} Pump fun transaction failed. Skipping", log_tag);
                            crawl_status_tx
                                .send(CrawlStatusOperation::MarkAsFailed(
                                    CrawlStatusRowIdentifier {
                                        account_address: pump_fun_program_address.to_string(),
                                        transaction_signature: pump_fun_program_signature.clone(),
                                    },
                                    "transaction failed".to_string(),
                                ))
                                .unwrap();
                            continue;
                        }

                        let message = if let EncodedTransaction::Json(tx_json) =
                            &tx.transaction.transaction
                        {
                            if let UiMessage::Raw(raw_message) = &tx_json.message {
                                let account_keys = &raw_message.account_keys;

                                if account_keys.contains(&pump_fun_program_address.to_string()) {
                                    Some((account_keys, raw_message))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        if message.is_none() {
                            println!("{} Pump fun transaction could not be parsed", log_tag);
                            crawl_status_tx
                                .send(CrawlStatusOperation::MarkAsFailed(
                                    CrawlStatusRowIdentifier {
                                        account_address: pump_fun_program_address.to_string(),
                                        transaction_signature: pump_fun_program_signature.clone(),
                                    },
                                    "transaction could not be parsed".to_string(),
                                ))
                                .unwrap();
                            continue;
                        }
                        let (_account_keys, raw_message) = message.unwrap();

                        let instructions_with_logs = parse_transaction_with_logs(
                            &tx.transaction.meta,
                            &raw_message,
                            &pump_fun_program_address,
                            &raydium_amm_program_address,
                        );

                        for (instruction_index, instruction_with_logs) in
                            instructions_with_logs.iter().enumerate()
                        {
                            let coin = match instruction_with_logs.instruction {
                                Some(Instruction::PumpFun(
                                    _,
                                    PumpFunInstruction::Create((_, coin)),
                                )) => coin,
                                Some(Instruction::PumpFun(
                                    _,
                                    PumpFunInstruction::Buy((_, coin)),
                                )) => coin,
                                Some(Instruction::PumpFun(
                                    _,
                                    PumpFunInstruction::Sell((_, coin)),
                                )) => coin,
                                _ => continue,
                            };
                            let (mint_address, _bonding_curve_address) = coin;

                            let mint_address_string = mint_address.to_string();
                            if already_seen_mint_addresses.insert(mint_address_string.clone()) {
                                pump_fun_tokens_tx.send(coin.clone()).unwrap();
                            }

                            for cpi_log in instruction_with_logs.cpi_logs.iter() {
                                let cpi_log = match cpi_log {
                                    CpiLog::PumpFun(cpi_log) => cpi_log,
                                };

                                if cpi_log.mint.to_string() != mint_address_string {
                                    // log is not for this mint
                                    continue;
                                }

                                let trade = TradeRow {
                                    coin_token_address: mint_address_string.clone(),
                                    price_coin_token_address: system_program_address.to_string(),
                                    transaction_signature: pump_fun_program_signature.clone(),
                                    slot: tx.slot,
                                    instruction_index: instruction_index as u64,
                                    block_time: OffsetDateTime::from_unix_timestamp(
                                        tx.block_time.unwrap(),
                                    )
                                    .unwrap(),
                                    coin_token_amount: cpi_log.token_amount,
                                    price_coin_token_amount: cpi_log.sol_amount,
                                    direction: if cpi_log.is_buy {
                                        TradeDirection::Buy
                                    } else {
                                        TradeDirection::Sell
                                    },
                                };

                                // TODO: dashset
                                trades_tx.send(trade).unwrap();
                            }
                        }

                        crawl_status_tx
                            .send(CrawlStatusOperation::MarkAsSucceeded(
                                CrawlStatusRowIdentifier {
                                    account_address: pump_fun_program_address.to_string(),
                                    transaction_signature: pump_fun_program_signature.clone(),
                                },
                            ))
                            .unwrap();
                    }
                }
            }
        });

        handles.push(handle);
    }

    handles
}
