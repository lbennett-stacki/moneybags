use crate::{
    constants::STORE_CONCURRENCY,
    db::client::db_client,
    trades::db::inserts::insert_trade,
    trades::db::table::TradeRow,
    utils::{blocking::blocking_call, log::log_time},
};
use crossbeam::channel::Receiver;
use std::thread;

const CONCURRENCY: usize = STORE_CONCURRENCY;

pub fn store_trades(trades_rx: &Receiver<TradeRow>) -> Vec<thread::JoinHandle<()>> {
    let mut handles = Vec::with_capacity(CONCURRENCY);

    for thread_index in 0..CONCURRENCY {
        let log_tag = format!(
            "                 {} store pump fun trades #{} | ",
            log_time(),
            thread_index
        );

        let trades_rx = trades_rx.clone();

        let handle = thread::spawn(move || {
            let db_client = db_client();

            while let Ok(trade) = trades_rx.recv() {
                println!(
                    "{} Storing trade for {}",
                    log_tag, trade.transaction_signature
                );

                let client = db_client.clone();
                // TODO: batching
                blocking_call(async move { insert_trade(&client, &trade).await.unwrap() });
            }
        });

        handles.push(handle);
    }

    handles
}
