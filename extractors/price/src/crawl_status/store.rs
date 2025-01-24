use crate::{
    constants::STORE_CONCURRENCY,
    crawl_status::{
        inserts::{mark_crawl_failed, mark_crawl_success, mark_first_account_signature},
        table::CrawlStatusOperation,
    },
    dragonfly::client::dragonfly_client,
    signatures::config::DEFAULT_SIGNATURES_LIMIT,
    utils::log::log_time,
};
use crossbeam::channel::Receiver;
use std::thread;

use super::inserts::insert_crawl_status;

const CONCURRENCY: usize = STORE_CONCURRENCY;

pub fn store_crawl_statuses(
    crawl_status_rx: &Receiver<CrawlStatusOperation>,
) -> Vec<thread::JoinHandle<()>> {
    let mut handles = Vec::with_capacity(CONCURRENCY);
    let batch_size = DEFAULT_SIGNATURES_LIMIT;

    for thread_index in 0..CONCURRENCY {
        let log_tag = format!("{} store crawl status #{} | ", log_time(), thread_index);
        let crawl_status_rx = crawl_status_rx.clone();

        let handle = thread::spawn(move || {
            let client = dragonfly_client();

            while let Ok(crawl_status) = crawl_status_rx.recv() {
                let result = match crawl_status {
                    CrawlStatusOperation::Create(crawl_status) => {
                        insert_crawl_status(&client, &crawl_status, batch_size)
                    }
                    CrawlStatusOperation::MarkAsSucceeded(transaction_signature) => {
                        mark_crawl_success(&client, &transaction_signature)
                    }
                    CrawlStatusOperation::MarkAsFailed(transaction_signature, error) => {
                        mark_crawl_failed(&client, &transaction_signature, &error)
                    }
                    CrawlStatusOperation::MarkAsFirstAccountSignature(transaction_signature) => {
                        mark_first_account_signature(&client, &transaction_signature)
                    }
                };

                match result {
                    Ok(_) => continue,
                    Err(e) => {
                        println!("{} Error storing crawl status: {:?}", log_tag, e);
                        panic!("Error storing crawl status: {:?}", e);
                    }
                }
            }
        });

        handles.push(handle);
    }

    handles
}
