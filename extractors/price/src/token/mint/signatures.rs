use crate::{
    crawl_status::{
        queries::{has_crawled_signature, CrawlStatusQueryError},
        table::{CrawlStatus, CrawlStatusOperation, CrawlStatusRow},
    },
    dragonfly::client::dragonfly_client,
    pump_fun::tokens::{MintAddress, PumpFunToken},
    rpc::{
        clients::get_rpc_nodes_count,
        pool::{RpcError, RpcPoolManager},
    },
    signatures::config::{build_signatures_config, build_signatures_window_config},
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
    crawl_status_tx: &Sender<CrawlStatusOperation>,
    pump_fun_tokens_rx: &Receiver<PumpFunToken>,
    pump_fun_tokens_tx: &Sender<PumpFunToken>,
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
        let pump_fun_tokens_tx = pump_fun_tokens_tx.clone();

        let rpc_pool_manager = rpc_pool_manager.clone();

        let handle = thread::spawn(move || {
            let dragonfly_client = dragonfly_client();

            while let Ok(pump_fun_token) = pump_fun_tokens_rx.recv() {
                let (mint_address, _bonding_curve_address) = pump_fun_token;
                let config = build_signatures_window_config(
                    &dragonfly_client,
                    &mint_address.to_string(),
                    None,
                );
                println!("{} Config: {:?}", log_tag, config);
                if let Err(ref config_err) = config {
                    if *config_err == CrawlStatusQueryError::HistoryComplete {
                        println!(
                            " {} Token mint history complete for {}",
                            log_tag, mint_address
                        );
                        continue;
                    }
                }
                let (oldest_signature, limit) = config.unwrap();

                println!(
                    "{} Running token mint crawl in historic mode starting from {:?} for {} signatures per batch",
                    log_tag, oldest_signature, limit
                );

                let signatures = rpc_pool_manager.execute(
                    |client| {
                        client.get_signatures_for_address_with_config(
                            &mint_address,
                            build_signatures_config(oldest_signature, None, Some(limit)),
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

                        let has_reached_first_account_signature = signatures_count < limit;

                        for (signature_index, signature) in signatures.iter().enumerate() {
                            let is_last_signature = signature_index == signatures_count - 1;
                            let is_first_account_signature =
                                has_reached_first_account_signature && is_last_signature;

                            if let Ok(has_crawled) =
                                has_crawled_signature(&dragonfly_client, &signature.signature)
                            {
                                if has_crawled {
                                    println!("{} has_crawled: {}", log_tag, has_crawled);
                                    println!(
                                        "{} is_first_account_signature: {} -- {}",
                                        log_tag, is_first_account_signature, signature.signature
                                    );

                                    if is_first_account_signature {
                                        crawl_status_tx
                                            .send(
                                                CrawlStatusOperation::MarkAsFirstAccountSignature(
                                                    signature.signature.clone(),
                                                ),
                                            )
                                            .unwrap();
                                    }
                                    continue;
                                }
                            }

                            let crawl_status = CrawlStatusRow {
                                account_address: mint_address.to_string(),
                                transaction_signature: signature.signature.clone(),
                                slot: signature.slot,
                                relative_transaction_index: signature_index as u64,
                                status: CrawlStatus::Pending,
                                is_first_account_signature,
                                error: None,
                            };
                            crawl_status_tx
                                .send(CrawlStatusOperation::Create(crawl_status))
                                .unwrap();
                            token_pump_fun_signatures_tx
                                .send((mint_address, signature.signature.clone()))
                                .unwrap();
                        }

                        if !has_reached_first_account_signature {
                            println!(
                                "{} There are more signatures to crawl. Resending token mint signatures crawl.",
                                log_tag
                            );
                            pump_fun_tokens_tx.send(pump_fun_token).unwrap();
                        }
                    }
                }
            }
        });

        handles.push(handle);
    }

    handles
}
