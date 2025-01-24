use crate::{
    constants::STORE_CONCURRENCY,
    db::client::db_client,
    raydium::{inserts::insert_raydium_pool, table::RaydiumPoolRow},
    utils::{blocking::blocking_call, log::log_time},
};
use crossbeam::channel::Receiver;
use std::thread;

const CONCURRENCY: usize = STORE_CONCURRENCY;

pub fn store_raydium_pools(trades_rx: &Receiver<RaydiumPoolRow>) -> Vec<thread::JoinHandle<()>> {
    let mut handles = Vec::with_capacity(CONCURRENCY);

    for thread_index in 0..CONCURRENCY {
        let log_tag = format!(
            "                 {} store raydium pools #{} | ",
            log_time(),
            thread_index
        );

        let trades_rx = trades_rx.clone();

        let handle = thread::spawn(move || {
            let db_client = db_client();

            while let Ok(pool) = trades_rx.recv() {
                println!(
                    "{} Storing raydium pool {} for mint {}",
                    log_tag, pool.pool_address, pool.mint_address
                );

                let client = db_client.clone();
                // TODO: batching
                blocking_call(async move { insert_raydium_pool(&client, &pool).await.unwrap() });
            }
        });

        handles.push(handle);
    }

    handles
}
