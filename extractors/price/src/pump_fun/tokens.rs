use super::program::signatures::TransactionSignature;
use crate::{
    cpi::cpi::CpiLog,
    crawl_status::queries::has_crawled_signature,
    instructions::{instruction::Instruction, parse::InstructionWithLogs},
    pump_fun::{errors::PumpFunTokenCrawlError, instructions::PumpFunInstruction},
    rpc::pool::RpcPoolManager,
    trades::db::table::{TradeDirection, TradeRow},
    transactions::{
        config::TRANSACTION_CONFIG, parse::parse_transaction_with_logs,
        status::is_failed_transaction,
    },
    utils::log::log_time,
};
use redis::Client;
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction, UiMessage,
};
use std::{collections::HashMap, str::FromStr};
use time::OffsetDateTime;

const CONCURRENCY: usize = 1;

pub type MintAddress = Pubkey;
pub type BondingCurveAddress = Pubkey;
pub type PumpFunToken = (MintAddress, BondingCurveAddress);

type Tokens = HashMap<MintAddress, (PumpFunToken, Vec<TradeRow>)>;

pub fn pump_fun_tokens_from_pump_fun_program_signature(
    rpc_pool_manager: &RpcPoolManager,
    dragonfly_client: &Client,
    pump_fun_program_signature: &TransactionSignature,
    pump_fun_program_address: &Pubkey,
    thread_index: u64,
    system_program_address: &Pubkey,
    raydium_amm_program_address: &Pubkey,
) -> Result<Tokens, PumpFunTokenCrawlError> {
    let log_tag = format!("{} Crawling pump fun transaction", log_time());

    println!("{} Crawling pump fun transaction", log_tag);
    if let Ok(has_crawled) = has_crawled_signature(&dragonfly_client, &pump_fun_program_signature) {
        if has_crawled {
            println!(
                "{} Pump fun program signature already crawled. Skipping",
                log_tag
            );
            return Err(PumpFunTokenCrawlError::AlreadyCrawled);
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
            return Err(PumpFunTokenCrawlError::TransactionFetchFailed(error));
        }
        Ok(tx) => {
            if is_failed_transaction(&tx) {
                return Err(PumpFunTokenCrawlError::TransactionFailed);
            }

            let message = if let EncodedTransaction::Json(tx_json) = &tx.transaction.transaction {
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
                return Err(PumpFunTokenCrawlError::TransactionMessageParseFailed);
            }
            let (_account_keys, raw_message) = message.unwrap();

            let instructions_with_logs = parse_transaction_with_logs(
                &tx.transaction.meta,
                &raw_message,
                &pump_fun_program_address,
                &raydium_amm_program_address,
            );

            let mut tokens: Tokens = Tokens::new();

            for (instruction_index, instruction_with_logs) in
                instructions_with_logs.iter().enumerate()
            {
                let token_with_trades = pump_fun_token_from_instruction_with_logs(
                    instruction_with_logs,
                    system_program_address,
                    &sig,
                    &tx,
                    instruction_index as u64,
                )?;
                let token = token_with_trades.0;
                let mint_address = token.0.clone();
                tokens.insert(mint_address, token_with_trades);
            }

            Ok(tokens)
        }
    }
}

fn pump_fun_token_from_instruction_with_logs(
    instruction_with_logs: &InstructionWithLogs,
    system_program_address: &Pubkey,
    pump_fun_program_signature: &Signature,
    tx: &EncodedConfirmedTransactionWithStatusMeta,
    instruction_index: u64,
) -> Result<(PumpFunToken, Vec<TradeRow>), PumpFunTokenCrawlError> {
    let found_token = match instruction_with_logs.instruction {
        Some(Instruction::PumpFun(_, PumpFunInstruction::Create((_, token)))) => token,
        Some(Instruction::PumpFun(_, PumpFunInstruction::Buy((_, token)))) => token,
        Some(Instruction::PumpFun(_, PumpFunInstruction::Sell((_, token)))) => token,
        _ => return Err(PumpFunTokenCrawlError::TokenNotFound),
    };

    let (mint_address, _bonding_curve_address) = found_token;
    let mint_address_string = mint_address.to_string();

    let mut trades = Vec::new();
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
            transaction_signature: pump_fun_program_signature.to_string(),
            slot: tx.slot,
            instruction_index: instruction_index as u64,
            block_time: OffsetDateTime::from_unix_timestamp(tx.block_time.unwrap()).unwrap(),
            coin_token_amount: cpi_log.token_amount,
            price_coin_token_amount: cpi_log.sol_amount,
            direction: if cpi_log.is_buy {
                TradeDirection::Buy
            } else {
                TradeDirection::Sell
            },
        };

        trades.push(trade);
    }

    Ok((found_token, trades))
}
