use crate::{
    constants::IS_HISTORIC_MODE,
    crawl_status::table::{CrawlStatus, CrawlStatusRow},
    db::client::db_client,
    rpc::pool::RpcPoolManager,
    signatures::config::{
        build_signatures_config, build_signatures_window_config, SignaturesWindowError,
        DEFAULT_SIGNATURES_LIMIT,
    },
    utils::log::log_time,
};
use crossbeam::channel::Sender;
use dashmap::DashSet;
use std::{sync::Arc, thread};

use super::program::get_pump_fun_program_address;

pub type TransactionSignature = String;

pub fn get_pump_fun_program_signatures(
    pump_fun_program_signatures_tx: &Sender<TransactionSignature>,
    crawl_status_tx: &Sender<CrawlStatusRow>,
    rpc_pool_manager: &RpcPoolManager,
) -> Vec<thread::JoinHandle<()>> {
    let concurrency = 1; // get_rpc_nodes_count();
    let mut handles = Vec::with_capacity(concurrency);

    let pump_fun_program_signatures_tx = pump_fun_program_signatures_tx.clone();
    let rpc_pool_manager = rpc_pool_manager.clone();

    let already_seen_signatures = Arc::new(DashSet::new());

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

        let already_seen_signatures = already_seen_signatures.clone();

        let handle = thread::spawn(move || loop {
            let db_client = db_client();

            let config = build_signatures_window_config(
                &db_client,
                &program_address.to_string(),
                Some(DEFAULT_SIGNATURES_LIMIT),
            );
            if let Err(config_err) = config {
                if config_err == SignaturesWindowError::HistoryComplete {
                    println!(" {} Pump fun program history complete", log_tag);
                    return;
                }
            }
            let (oldest_signature, latest_signature, limit) = config.unwrap();

            if IS_HISTORIC_MODE {
                println!(
                    "{} Running pump fun program crawl in historic mode starting from {:?} for {} signatures per batch",
                    log_tag, oldest_signature, limit
                );
            } else {
                println!(
                    "{} Running pump fun program crawl in latest mode up to {:?} for {} signatures per batch",
                    log_tag, latest_signature, limit
                );
            }

            let signatures = rpc_pool_manager.execute(
                |client| {
                    client.get_signatures_for_address_with_config(
                        &program_address,
                        build_signatures_config(oldest_signature, latest_signature, Some(limit)),
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

                    println!("{} Got signatures ({})", log_tag, signatures_count);

                    for (signature_index, signature) in signatures.iter().enumerate() {
                        if already_seen_signatures.insert(signature.signature.clone()) {
                            let crawl_status = CrawlStatusRow {
                                account_address: program_address.to_string(),
                                transaction_signature: signature.signature.clone(),
                                slot: signature.slot,
                                status: CrawlStatus::Pending,
                                is_first_account_signature: signatures_count < limit
                                    && signature_index == signatures_count - 1,
                            };
                            crawl_status_tx.send(crawl_status).unwrap();

                            pump_fun_program_signatures_tx
                                .send(signature.signature.clone())
                                .unwrap();
                        }
                    }
                }
            };
        });

        handles.push(handle);
    }

    handles
}
