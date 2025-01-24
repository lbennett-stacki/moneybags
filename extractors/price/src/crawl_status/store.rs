use crate::{
    constants::STORE_CONCURRENCY,
    crawl_status::{inserts::insert_crawl_status, table::CrawlStatusRow},
    dragonfly::client::dragonfly_client,
    utils::log::log_time,
};
use crossbeam::channel::Receiver;
use std::thread;

const CONCURRENCY: usize = STORE_CONCURRENCY;

pub fn store_crawl_statuses(
    crawl_status_rx: &Receiver<CrawlStatusRow>,
) -> Vec<thread::JoinHandle<()>> {
    let mut handles = Vec::with_capacity(CONCURRENCY);

    for thread_index in 0..CONCURRENCY {
        let log_tag = format!("{} store crawl status #{} | ", log_time(), thread_index);
        let crawl_status_rx = crawl_status_rx.clone();

        let handle = thread::spawn(move || {
            let client = dragonfly_client();

            while let Ok(crawl_status) = crawl_status_rx.recv() {
                println!(
                    "{} Storing crawl status for tx {}",
                    log_tag, crawl_status.transaction_signature
                );

                if let Err(e) = insert_crawl_status(&client, &crawl_status) {
                    println!("{} Error storing crawl status: {:?}", log_tag, e);
                }
            }
        });

        handles.push(handle);
    }

    handles
}
