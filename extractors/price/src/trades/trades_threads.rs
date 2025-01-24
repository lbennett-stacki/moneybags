use super::{errors::TradeCrawlError, trades::token_trade_from_transaction};
use crate::{
    crawl_status::{
        channels::{mark_as_failed, mark_as_succeeded},
        table::CrawlStatusOperation,
    },
    dragonfly::client::dragonfly_client,
    pump_fun::program::program::get_pump_fun_program_address,
    raydium::amm::get_raydium_amm_program_address,
    rpc::{clients::get_rpc_nodes_count, pool::RpcPoolManager},
    termination::{is_terminated, terminate, terminate_on_error, TerminationFlag},
    token::mint::signatures::TokenMintSignatures,
    trades::db::table::TradeRow,
    utils::log::log_time,
};
use crossbeam::channel::{Receiver, Sender};
use std::thread;

pub fn token_trades_threads(
    trades_tx: &Sender<TradeRow>,
    token_pump_fun_signatures_rx: &Receiver<TokenMintSignatures>,
    rpc_pool_manager: &RpcPoolManager,
    crawl_status_tx: &Sender<CrawlStatusOperation>,
    termination_flag: &TerminationFlag,
) -> Vec<thread::JoinHandle<()>> {
    let concurrency = get_rpc_nodes_count();

    let mut handles = Vec::with_capacity(concurrency);

    let pump_fun_program_address = get_pump_fun_program_address();
    let raydium_amm_program_address = get_raydium_amm_program_address();

    for thread_index in 0..concurrency {
        let log_tag = format!(
            "             {} token trades from transactions #{} | ",
            log_time(),
            thread_index
        );

        let trades_tx = trades_tx.clone();
        let token_pump_fun_signatures_rx = token_pump_fun_signatures_rx.clone();
        let rpc_pool_manager = rpc_pool_manager.clone();
        let crawl_status_tx = crawl_status_tx.clone();
        let dragonfly_client = dragonfly_client();
        let termination_flag = termination_flag.clone();

        let handle = thread::spawn(move || {
            while let Ok((_mint_address, token_tx_signature)) = token_pump_fun_signatures_rx.recv()
            {
                if is_terminated(&termination_flag) {
                    println!("{} Termination flag set. Exiting", log_tag);
                    break;
                }

                let result = token_trade_from_transaction(
                    &rpc_pool_manager,
                    &crawl_status_tx,
                    &dragonfly_client,
                    thread_index,
                    &token_tx_signature,
                    &pump_fun_program_address,
                    &raydium_amm_program_address,
                );

                match result {
                    Ok(trades) => {
                        for trade in trades {
                            terminate_on_error(&termination_flag, trades_tx.send(trade));
                        }

                        terminate_on_error(
                            &termination_flag,
                            mark_as_succeeded(&crawl_status_tx, &token_tx_signature),
                        );
                    }
                    Err(TradeCrawlError::AlreadyCrawled) => {
                        println!("{} Transaction already crawled. Skipping", log_tag);
                        continue;
                    }
                    Err(TradeCrawlError::TransactionFailed) => {
                        println!("{} Transaction failed. Skipping", log_tag);
                        terminate_on_error(
                            &termination_flag,
                            mark_as_succeeded(&crawl_status_tx, &token_tx_signature),
                        );
                        continue;
                    }
                    Err(err @ TradeCrawlError::TransactionMessageParseFailed)
                    | Err(err @ TradeCrawlError::TransactionFetchFailed)
                    | Err(err @ TradeCrawlError::CrawlStatusSend(_))
                    | Err(err @ TradeCrawlError::BlockTimeParseError(_)) => {
                        println!("{} Token transaction could not be parsed", log_tag);
                        terminate_on_error(
                            &termination_flag,
                            mark_as_failed(&crawl_status_tx, &token_tx_signature, &err.to_string()),
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
