use super::program::{program::get_pump_fun_program_address, signatures::TransactionSignature};
use crate::{
    crawl_status::{
        channels::{mark_as_failed, mark_as_succeeded},
        table::CrawlStatusOperation,
    },
    dragonfly::client::dragonfly_client,
    pump_fun::{
        errors::PumpFunTokenCrawlError, tokens::pump_fun_tokens_from_pump_fun_program_signature,
    },
    raydium::amm::get_raydium_amm_program_address,
    rpc::{clients::get_rpc_nodes_count, pool::RpcPoolManager},
    system::program::get_system_program_address,
    termination::{terminate, terminate_on_error, TerminationFlag},
    trades::db::table::TradeRow,
    utils::log::log_time,
};
use crossbeam::channel::{Receiver, Sender};
use solana_sdk::pubkey::Pubkey;
use std::thread;

const CONCURRENCY: usize = 1;

pub type MintAddress = Pubkey;
pub type BondingCurveAddress = Pubkey;
pub type PumpFunToken = (MintAddress, BondingCurveAddress);

pub fn pump_fun_tokens_threads(
    pump_fun_tokens_tx: &Sender<PumpFunToken>,
    trades_tx: &Sender<TradeRow>,
    pump_fun_program_signatures_rx: &Receiver<TransactionSignature>,
    crawl_status_tx: &Sender<CrawlStatusOperation>,
    rpc_pool_manager: &RpcPoolManager,
    termination_flag: &TerminationFlag,
) -> Vec<thread::JoinHandle<()>> {
    let concurrency = get_rpc_nodes_count();
    let mut handles = Vec::with_capacity(concurrency);

    let pump_fun_program_address = get_pump_fun_program_address();
    let system_program_address = get_system_program_address();

    let raydium_amm_program_address = get_raydium_amm_program_address();

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
        let dragonfly_client = dragonfly_client();
        let termination_flag = termination_flag.clone();

        let handle = thread::spawn(move || {
            while let Ok(pump_fun_program_signature) = pump_fun_signatures_rx.recv() {
                println!("{} Crawling pump fun transaction", log_tag);
                let tokens_with_trades = pump_fun_tokens_from_pump_fun_program_signature(
                    &rpc_pool_manager,
                    &dragonfly_client,
                    &pump_fun_program_signature,
                    &pump_fun_program_address,
                    thread_index as u64,
                    &system_program_address,
                    &raydium_amm_program_address,
                );

                match tokens_with_trades {
                    Ok(tokens_with_trades) => {
                        for (token, trades) in tokens_with_trades.values() {
                            terminate_on_error(&termination_flag, pump_fun_tokens_tx.send(*token));
                            for trade in trades {
                                terminate_on_error(
                                    &termination_flag,
                                    trades_tx.send(trade.clone()),
                                );
                            }
                        }
                        terminate_on_error(
                            &termination_flag,
                            mark_as_succeeded(&crawl_status_tx, &pump_fun_program_signature),
                        );
                    }
                    Err(PumpFunTokenCrawlError::AlreadyCrawled) => {
                        println!("{} Transaction already crawled. Skipping", log_tag);
                        continue;
                    }
                    Err(PumpFunTokenCrawlError::TransactionFailed) => {
                        println!("{} Transaction failed. Skipping", log_tag);
                        terminate_on_error(
                            &termination_flag,
                            mark_as_succeeded(&crawl_status_tx, &pump_fun_program_signature),
                        );
                        continue;
                    }
                    Err(err @ PumpFunTokenCrawlError::TransactionMessageParseFailed)
                    | Err(err @ PumpFunTokenCrawlError::TransactionFetchFailed(_))
                    | Err(err @ PumpFunTokenCrawlError::CrawlStatusSend(_))
                    | Err(err @ PumpFunTokenCrawlError::BlockTimeParseError(_))
                    | Err(err @ PumpFunTokenCrawlError::TokenNotFound) => {
                        println!("{} Token transaction could not be parsed", log_tag);
                        terminate_on_error(
                            &termination_flag,
                            mark_as_failed(
                                &crawl_status_tx,
                                &pump_fun_program_signature,
                                &err.to_string(),
                            ),
                        );
                        terminate(&termination_flag);
                    }
                }
            }
        });

        handles.push(handle);
    }

    handles
}
