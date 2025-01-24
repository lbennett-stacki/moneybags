use crate::{
    constants::STORE_CONCURRENCY,
    db::client::db_client,
    token::{inserts::insert_token_decimals, table::TokenRow},
    utils::{blocking::blocking_call, log::log_time},
};
use crossbeam::channel::Receiver;
use std::thread;

const CONCURRENCY: usize = STORE_CONCURRENCY;

pub fn store_tokens(token_accounts_rx: &Receiver<TokenRow>) -> Vec<thread::JoinHandle<()>> {
    let mut handles = Vec::with_capacity(CONCURRENCY);

    for thread_index in 0..CONCURRENCY {
        let log_tag = format!(
            "                 {} store token #{} | ",
            log_time(),
            thread_index
        );

        let token_accounts_rx = token_accounts_rx.clone();

        let handle = thread::spawn(move || {
            let db_client = db_client();

            while let Ok(token) = token_accounts_rx.recv() {
                println!("{} Storing token for {}", log_tag, token.mint_address);

                let client = db_client.clone();
                blocking_call(async move { insert_token_decimals(&client, &token).await.unwrap() });
            }
        });

        handles.push(handle);
    }

    handles
}
