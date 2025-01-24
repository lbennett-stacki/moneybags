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
use redis::RedisError;
use std::thread;

use super::inserts::insert_crawl_status;

const CONCURRENCY: usize = STORE_CONCURRENCY;

#[derive(Debug)]
pub enum CrawlStatusError {
    CannotUpdateToPendingStatus,
    Redis(RedisError),
}

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
                            .map_err(|e| CrawlStatusError::Redis(e))
                    }
                    CrawlStatusOperation::MarkAsSucceeded(crawl_status) => {
                        mark_crawl_success(&client, &crawl_status.transaction_signature)
                            .map_err(|e| CrawlStatusError::Redis(e))
                    }
                    CrawlStatusOperation::MarkAsFailed(crawl_status, error) => {
                        mark_crawl_failed(&client, &crawl_status.transaction_signature, &error)
                            .map_err(|e| CrawlStatusError::Redis(e))
                    }
                    CrawlStatusOperation::MarkAsFirstAccountSignature(crawl_status) => {
                        mark_first_account_signature(&client, &crawl_status.transaction_signature)
                            .map_err(|e| CrawlStatusError::Redis(e))
                    }
                };

                match result {
                    Ok(_) => {}
                    Err(e) => println!("{} Error storing crawl status: {:?}", log_tag, e),
                }
            }
        });

        handles.push(handle);
    }

    handles
}
