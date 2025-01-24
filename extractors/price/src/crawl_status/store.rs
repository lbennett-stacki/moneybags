use crate::{
    constants::STORE_CONCURRENCY,
    crawl_status::{inserts::insert_crawl_status, table::CrawlStatusRow},
    db::client::db_client,
    utils::blocking::blocking_call,
};
use crossbeam::channel::Receiver;
use std::thread;

const CONCURRENCY: usize = STORE_CONCURRENCY;

pub fn store_crawl_statuses(
    crawl_status_rx: &Receiver<CrawlStatusRow>,
) -> Vec<thread::JoinHandle<()>> {
    let mut handles = Vec::with_capacity(CONCURRENCY);

    for _thread_index in 0..CONCURRENCY {
        let crawl_status_rx = crawl_status_rx.clone();

        let handle = thread::spawn(move || {
            let db_client = db_client();

            while let Ok(crawl_status) = crawl_status_rx.recv() {
                let client = db_client.clone();
                // TODO: batching
                blocking_call(
                    async move { insert_crawl_status(&client, &crawl_status).await.unwrap() },
                );
            }
        });

        handles.push(handle);
    }

    handles
}
