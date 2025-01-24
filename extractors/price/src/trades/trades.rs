use super::errors::TradeCrawlError;
use crate::{
    crawl_status::{queries::has_crawled_signature, table::CrawlStatusOperation},
    instructions::instruction::Instruction,
    pump_fun::trades::trade_from_pump_fun_instruction,
    raydium::trades::trade_from_raydium_instruction,
    rpc::pool::{RpcError, RpcPoolManager},
    trades::db::table::TradeRow,
    transactions::{
        config::TRANSACTION_CONFIG, parse::parse_transaction_with_logs,
        status::is_failed_transaction,
    },
    utils::log::log_time,
};
use crossbeam::channel::Sender;
use redis::Client;
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction, UiMessage,
};
use std::str::FromStr;

fn parse_transaction(
    tx: &EncodedConfirmedTransactionWithStatusMeta,
    token_tx_signature: &str,
    crawl_status_tx: &Sender<CrawlStatusOperation>,
    pump_fun_program_address: &Pubkey,
    raydium_amm_program_address: &Pubkey,
) -> Result<Vec<TradeRow>, TradeCrawlError> {
    if is_failed_transaction(&tx) {
        return Err(TradeCrawlError::TransactionFailed);
    }

    println!(
        "{} Got token transaction for {} with slot {} and block time {:?}",
        log_time(),
        token_tx_signature,
        tx.slot,
        tx.block_time
    );

    let message = if let EncodedTransaction::Json(tx_json) = &tx.transaction.transaction {
        if let UiMessage::Raw(raw_message) = &tx_json.message {
            Some(raw_message)
        } else {
            None
        }
    } else {
        None
    };

    if message.is_none() {
        return Err(TradeCrawlError::TransactionMessageParseFailed);
    }
    let raw_message = message.unwrap();

    let instructions_with_logs = parse_transaction_with_logs(
        &tx.transaction.meta,
        &raw_message,
        &pump_fun_program_address,
        &raydium_amm_program_address,
    );

    let mut trades = Vec::new();

    for instruction_with_logs in instructions_with_logs.iter() {
        let parsed_trade = match &instruction_with_logs.instruction {
            Some(Instruction::PumpFun(instruction_index, instruction)) => {
                let trade = trade_from_pump_fun_instruction(
                    &instruction,
                    &instruction_index,
                    tx.slot,
                    tx.block_time.unwrap() as u64,
                    &instruction_with_logs.cpi_logs,
                    &token_tx_signature,
                )?;

                match trade {
                    Some(trade) => trade,
                    None => continue,
                }
            }
            Some(Instruction::Raydium(instruction_index, instruction)) => {
                trade_from_raydium_instruction(
                    &instruction,
                    &instruction_index,
                    tx.slot,
                    tx.block_time.unwrap() as u64,
                    &token_tx_signature,
                )
            }
            None => continue,
        };

        trades.push(parsed_trade);
    }

    Ok(trades)
}

pub fn token_trade_from_transaction(
    rpc_pool_manager: &RpcPoolManager,
    crawl_status_tx: &Sender<CrawlStatusOperation>,
    dragonfly_client: &Client,
    thread_index: usize,
    token_tx_signature: &str,
    pump_fun_program_address: &Pubkey,
    raydium_amm_program_address: &Pubkey,
) -> Result<Vec<TradeRow>, TradeCrawlError> {
    let log_tag = format!(
        "{} token pump fun transactions #{} | ",
        log_time(),
        thread_index
    );

    if let Ok(has_crawled) = has_crawled_signature(&dragonfly_client, &token_tx_signature) {
        if has_crawled {
            return Err(TradeCrawlError::AlreadyCrawled);
        }
    }

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
            return Err(TradeCrawlError::TransactionFetchFailed);
        }
        Ok(tx) => {
            let trades = parse_transaction(
                &tx,
                token_tx_signature,
                crawl_status_tx,
                pump_fun_program_address,
                raydium_amm_program_address,
            )?;
            return Ok(trades);
        }
    }
}
