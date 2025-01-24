use crate::{
    crawl_status::{
        queries::{has_crawled_signature, CrawlStatusQueryError},
        table::{CrawlStatus, CrawlStatusOperation, CrawlStatusRow},
    },
    dragonfly::client::dragonfly_client,
    rpc::pool::RpcPoolManager,
    signatures::config::{
        build_signatures_config, build_signatures_window_config, DEFAULT_SIGNATURES_LIMIT,
    },
    utils::log::log_time,
};
use crossbeam::channel::Sender;
use std::thread;

use super::program::get_pump_fun_program_address;

pub type TransactionSignature = String;

pub fn get_pump_fun_program_signatures(
    pump_fun_program_signatures_tx: &Sender<TransactionSignature>,
    crawl_status_tx: &Sender<CrawlStatusOperation>,
    rpc_pool_manager: &RpcPoolManager,
) -> Vec<thread::JoinHandle<()>> {
    let concurrency = 1; // get_rpc_nodes_count();
    let mut handles = Vec::with_capacity(concurrency);

    let pump_fun_program_signatures_tx = pump_fun_program_signatures_tx.clone();
    let rpc_pool_manager = rpc_pool_manager.clone();

    let program_address = get_pump_fun_program_address();

    for thread_index in 0..concurrency {
        let log_tag = format!(
            "     {} pump program signatures #{} | ",
            log_time(),
            thread_index
        );

        let pump_fun_program_signatures_tx = pump_fun_program_signatures_tx.clone();
        let crawl_status_tx = crawl_status_tx.clone();

        let rpc_pool_manager = rpc_pool_manager.clone();

        let handle = thread::spawn(move || loop {
            let dragonfly_client = dragonfly_client();

            let config = build_signatures_window_config(
                &dragonfly_client,
                &program_address.to_string(),
                Some(DEFAULT_SIGNATURES_LIMIT),
            );
            if let Err(ref config_err) = config {
                if *config_err == CrawlStatusQueryError::HistoryComplete {
                    println!(" {} Pump fun program history complete", log_tag);
                    return;
                }
            }
            let (oldest_signature, limit) = config.unwrap();

            println!(
                "{} Running pump fun program crawl in historic mode starting from {:?} for {} signatures per batch",
                log_tag, oldest_signature, limit
            );

            let signatures = rpc_pool_manager.execute(
                |client| {
                    client.get_signatures_for_address_with_config(
                        &program_address,
                        build_signatures_config(oldest_signature, None, Some(limit)),
                    )
                },
                Some(thread_index as u64),
            );

            match signatures {
                Err(error) => {
                    println!(
                        "{} Error getting signature.\n{:?}\nSkipping",
                        log_tag, error
                    );
                    return;
                }
                Ok(signatures) => {
                    let signatures_count = signatures.len();

                    println!(
                        "{} Got pump fun program signatures ({})",
                        log_tag, signatures_count
                    );

                    let is_last_batch = signatures_count < limit;

                    for (signature_index, signature) in signatures.iter().enumerate() {
                        if let Ok(has_crawled) =
                            has_crawled_signature(&dragonfly_client, &signature.signature)
                        {
                            if has_crawled {
                                println!(
                                    "{} Signature already crawled ({})",
                                    log_tag, signature.signature
                                );
                                continue;
                            }
                        }

                        let is_last_signature = signature_index == signatures_count - 1;

                        println!("{} Processing signature ({})", log_tag, signature.signature);

                        let crawl_status = CrawlStatusRow {
                            account_address: program_address.to_string(),
                            transaction_signature: signature.signature.clone(),
                            slot: signature.slot,
                            relative_transaction_index: signature_index as u64,
                            status: CrawlStatus::Pending,
                            is_first_account_signature: is_last_batch && is_last_signature,
                            error: None,
                        };
                        crawl_status_tx
                            .send(CrawlStatusOperation::Create(crawl_status))
                            .unwrap();

                        pump_fun_program_signatures_tx
                            .send(signature.signature.clone())
                            .unwrap();
                    }
                }
            };
        });

        handles.push(handle);
    }

    handles
}
