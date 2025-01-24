use crate::{
    constants::IS_HISTORIC_MODE,
    crawl_status::table::{CrawlStatus, CrawlStatusRow},
    db::client::db_client,
    pump_fun::tokens::{MintAddress, PumpFunToken},
    rpc::{
        clients::get_rpc_nodes_count,
        pool::{RpcError, RpcPoolManager},
    },
    signatures::config::{
        build_signatures_config, build_signatures_window_config, SignaturesWindowError,
    },
    utils::log::log_time,
};
use crossbeam::channel::{Receiver, Sender};
use std::thread;

pub type TokenMintSignatures = (MintAddress, String);

// TODO: whilst we are crawling for mint addresses in signatures from the pump fun program,
// we will come across various trades that we should send to trades_tx channel sender
// als we will find raydium pools that we should send to raydium_pools_tx channel sender
pub fn get_token_mint_signatures(
    token_pump_fun_signatures_tx: &Sender<TokenMintSignatures>,
    crawl_status_tx: &Sender<CrawlStatusRow>,
    pump_fun_tokens_rx: &Receiver<PumpFunToken>,
    rpc_pool_manager: &RpcPoolManager,
) -> Vec<thread::JoinHandle<()>> {
    let concurrency = get_rpc_nodes_count();
    let mut handles = Vec::with_capacity(concurrency);

    for thread_index in 0..concurrency {
        let log_tag = format!(
            "         {} token signatures #{} | ",
            log_time(),
            thread_index
        );

        let token_pump_fun_signatures_tx = token_pump_fun_signatures_tx.clone();
        let crawl_status_tx = crawl_status_tx.clone();
        let pump_fun_tokens_rx = pump_fun_tokens_rx.clone();

        let rpc_pool_manager = rpc_pool_manager.clone();

        let handle = thread::spawn(move || {
            let db_client = db_client();

            while let Ok((mint_address, _bonding_curve_address)) = pump_fun_tokens_rx.recv() {
                let db_client = db_client.clone();

                let config =
                    build_signatures_window_config(&db_client, &mint_address.to_string(), None);
                if let Err(config_err) = config {
                    if config_err == SignaturesWindowError::HistoryComplete {
                        println!(
                            " {} Token mint history complete for {}",
                            log_tag, mint_address
                        );
                        continue;
                    }
                }
                let (oldest_signature, latest_signature, limit) = config.unwrap();

                if IS_HISTORIC_MODE {
                    println!(
                        "{} Running token mint crawl in historic mode starting from {:?} for {} signatures per batch",
                        log_tag, oldest_signature, limit
                );
                } else {
                    println!(
                        "{} Running token mint crawl in latest mode up to {:?} for {} signatures per batch",
                        log_tag, latest_signature, limit
                    );
                }

                let signatures = rpc_pool_manager.execute(
                    |client| {
                        client.get_signatures_for_address_with_config(
                            &mint_address,
                            build_signatures_config(
                                oldest_signature,
                                latest_signature,
                                Some(limit),
                            ),
                        )
                    },
                    Some(thread_index as u64),
                );

                match signatures {
                    Err(RpcError::ClientError(error)) => {
                        println!(
                            "{} Error getting token signatures.\n{:?}\nSkipping",
                            log_tag, error
                        );
                        continue;
                    }
                    Ok(signatures) => {
                        let signatures_count = signatures.len();

                        println!(
                            "{} Got token signatures ({}) for {}",
                            log_tag, signatures_count, mint_address
                        );

                        for (signature_index, signature) in signatures.iter().enumerate() {
                            let is_first_account_signature =
                                signatures_count < limit && signature_index == signatures_count - 1;
                            let crawl_status = CrawlStatusRow {
                                account_address: mint_address.to_string(),
                                transaction_signature: signature.signature.clone(),
                                slot: signature.slot,
                                status: CrawlStatus::Pending,
                                is_first_account_signature,
                            };
                            crawl_status_tx.send(crawl_status).unwrap();
                            token_pump_fun_signatures_tx
                                .send((mint_address, signature.signature.clone()))
                                .unwrap();
                        }
                    }
                }
            }
        });

        handles.push(handle);
    }

    handles
}
