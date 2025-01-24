use crate::{
    cpi::cpi::CpiLog,
    instructions::{
        discriminators::build_instruction_discriminators,
        instruction::{Instruction, InstructionIndex},
    },
    pump_fun::{instructions::PumpFunInstruction, program::program::get_pump_fun_program_address},
    raydium::{table::PoolAddress, trades::get_trades_from_raydium_instruction},
    rpc::{
        clients::get_rpc_nodes_count,
        pool::{RpcError, RpcPoolManager},
    },
    token::mint::signatures::TokenMintSignatures,
    trades::table::{TradeDirection, TradeRow},
    transactions::{
        config::TRANSACTION_CONFIG, parse::parse_transaction_with_logs,
        status::is_failed_transaction,
    },
    utils::log::log_time,
};
use crossbeam::channel::{Receiver, Sender};
use solana_sdk::signature::Signature;
use solana_transaction_status::{EncodedTransaction, UiMessage};
use std::{str::FromStr, thread};
use time::OffsetDateTime;

pub fn get_token_pump_fun_trades(
    trades_tx: &Sender<TradeRow>,
    token_pump_fun_signatures_rx: &Receiver<TokenMintSignatures>,
    rpc_pool_manager: &RpcPoolManager,
    raydium_pools_tx: &Sender<PoolAddress>,
) -> Vec<thread::JoinHandle<()>> {
    let concurrency = get_rpc_nodes_count();

    let mut handles = Vec::with_capacity(concurrency);

    let pump_fun_program_address = get_pump_fun_program_address();

    for thread_index in 0..concurrency {
        let log_tag = format!(
            "             {} token pump fun transactions #{} | ",
            log_time(),
            thread_index
        );

        let trades_tx = trades_tx.clone();
        let raydium_pools_tx = raydium_pools_tx.clone();
        let token_pump_fun_signatures_rx = token_pump_fun_signatures_rx.clone();
        let rpc_pool_manager = rpc_pool_manager.clone();
        let known_discriminators = build_instruction_discriminators();

        let handle = thread::spawn(move || {
            while let Ok((_mint_address, token_tx_signature)) = token_pump_fun_signatures_rx.recv()
            {
                println!(
                    "{} Getting token transaction for {}",
                    log_tag, token_tx_signature
                );
                let tx = rpc_pool_manager.execute(
                    |client| {
                        client.get_transaction_with_config(
                            &Signature::from_str(&token_tx_signature).unwrap(),
                            TRANSACTION_CONFIG,
                        )
                    },
                    Some(thread_index as u64),
                );

                match tx {
                    Err(RpcError::ClientError(error)) => {
                        println!(
                            "{} Error getting token transaction.\n{:?}\nSkipping",
                            log_tag, error
                        );
                        continue;
                    }
                    Ok(tx) => {
                        if is_failed_transaction(&tx) {
                            println!("{} Token transaction failed. Skipping", log_tag);
                            continue;
                        }

                        println!(
                            "{} Got token transaction for {} with slot {} and block time {:?}",
                            log_tag, token_tx_signature, tx.slot, tx.block_time
                        );

                        if let EncodedTransaction::Json(tx_json) = &tx.transaction.transaction {
                            if let UiMessage::Raw(raw_message) = &tx_json.message {
                                let account_keys = &raw_message.account_keys;

                                if !account_keys.contains(&pump_fun_program_address.to_string()) {
                                    // not a transaction executed on pump fun prog
                                    println!("NOT A PUMP FUN PROGRAM TRANSACTION, BUT WE WILL TRY ANYWAYS");
                                    // TODO: decide if we need this
                                    // continue;
                                }

                                println!("PARSING TX LOGS");
                                let instructions_with_logs = parse_transaction_with_logs(
                                    &tx.transaction.meta,
                                    &known_discriminators,
                                    &account_keys,
                                    &raw_message,
                                    &pump_fun_program_address,
                                );

                                println!(
                                    "Now we gots instructions with loginis {:#?} ",
                                    instructions_with_logs
                                );

                                for instruction_with_logs in instructions_with_logs.iter() {
                                    match &instruction_with_logs.instruction {
                                        Some(Instruction::PumpFun(
                                            instruction_index,
                                            instruction,
                                        )) => get_trades_from_pump_fun_instruction(
                                            &instruction,
                                            &instruction_index,
                                            tx.slot,
                                            tx.block_time.unwrap() as u64,
                                            &instruction_with_logs.cpi_logs,
                                            &token_tx_signature,
                                            &trades_tx,
                                        ),
                                        Some(Instruction::Raydium(
                                            instruction_index,
                                            instruction,
                                        )) => get_trades_from_raydium_instruction(
                                            &instruction,
                                            &instruction_index,
                                            tx.slot,
                                            tx.block_time.unwrap() as u64,
                                            &instruction_with_logs.cpi_logs,
                                            &token_tx_signature,
                                            &trades_tx,
                                            &raydium_pools_tx,
                                        ),
                                        None => continue,
                                    }
                                }
                            } else {
                                println!(
                                    "NOT A RAW MESSAGE FOR PUMP FUN PROGRAM TRANSACTION, SKIPPING"
                                );
                            }
                        } else {
                            println!("NOT AN ENCODED PUMP FUN PROGRAM TRANSACTION, SKIPPING");
                        }
                    }
                }
            }
        });

        handles.push(handle);
    }

    handles
}

fn get_trades_from_pump_fun_instruction(
    instruction: &PumpFunInstruction,
    instruction_index: &InstructionIndex,
    slot: u64,
    block_time: u64,
    cpi_logs: &Vec<CpiLog>,
    token_tx_signature: &String,
    trades_tx: &Sender<TradeRow>,
) {
    let (discovered_mint_address, _bonding_curve_address) = match instruction {
        PumpFunInstruction::Create((_, pump_fun_token)) => pump_fun_token,
        PumpFunInstruction::Buy((_, pump_fun_token)) => pump_fun_token,
        PumpFunInstruction::Sell((_, pump_fun_token)) => pump_fun_token,
    };
    let discovered_mint_address_string = discovered_mint_address.to_string();

    for cpi_log in cpi_logs.iter() {
        if let CpiLog::PumpFun(cpi_log) = cpi_log {
            if cpi_log.mint.to_string() != discovered_mint_address_string {
                // log is not for this mint
                continue;
            }

            let trade = TradeRow {
                coin_token_address: discovered_mint_address_string.clone(),
                price_coin_token_address: "11111111111111111111111111111111".to_string(),
                transaction_signature: token_tx_signature.clone(),
                slot,
                instruction_index: *instruction_index,
                block_time: OffsetDateTime::from_unix_timestamp(block_time as i64).unwrap(),
                coin_token_amount: cpi_log.token_amount,
                price_coin_token_amount: cpi_log.sol_amount,
                direction: if cpi_log.is_buy {
                    TradeDirection::Buy
                } else {
                    TradeDirection::Sell
                },
            };

            trades_tx.send(trade).unwrap();
        }
    }
}
