use super::errors::PumpFunProgramSignaturesError;
use crate::{
    crawl_status::{
        queries::has_crawled_signature,
        table::{CrawlStatus, CrawlStatusRow},
    },
    rpc::pool::RpcPoolManager,
    signatures::config::{
        build_signatures_config, build_signatures_window_config, DEFAULT_SIGNATURES_LIMIT,
    },
    utils::log::log_time,
};
use redis::Client;
use solana_sdk::pubkey::Pubkey;

pub type TransactionSignature = String;

pub fn get_pump_fun_program_signatures(
    rpc_pool_manager: &RpcPoolManager,
    dragonfly_client: &Client,
    program_address: &Pubkey,
    thread_index: u64,
) -> Result<Vec<(TransactionSignature, CrawlStatusRow)>, PumpFunProgramSignaturesError> {
    let (oldest_signature, limit) = build_signatures_window_config(
        &dragonfly_client,
        &program_address.to_string(),
        Some(DEFAULT_SIGNATURES_LIMIT),
    )
    .map_err(PumpFunProgramSignaturesError::GetWindowConfigFailed)?;

    println!(
                "{} Running pump fun program crawl in historic mode starting from {:?} for {} signatures per batch",
                log_time(), oldest_signature, limit
            );

    let signatures = rpc_pool_manager
        .execute(
            |client| {
                client.get_signatures_for_address_with_config(
                    &program_address,
                    build_signatures_config(oldest_signature, None, Some(limit)),
                )
            },
            Some(thread_index as u64),
        )
        .map_err(PumpFunProgramSignaturesError::GetSignaturesFailed)?;

    let signatures_count = signatures.len();

    println!(
        "{} Got pump fun program signatures ({})",
        log_time(),
        signatures_count
    );

    let is_last_batch = signatures_count < limit;

    let mut signatures_and_statuses = Vec::new();

    for (signature_index, signature) in signatures.iter().enumerate() {
        if let Ok(has_crawled) = has_crawled_signature(&dragonfly_client, &signature.signature) {
            if has_crawled {
                println!(
                    "{} Signature already crawled ({})",
                    log_time(),
                    signature.signature
                );
                continue;
            }
        }

        let is_last_signature = signature_index == signatures_count - 1;

        println!(
            "{} Processing signature ({})",
            log_time(),
            signature.signature
        );

        let crawl_status = CrawlStatusRow {
            account_address: program_address.to_string(),
            transaction_signature: signature.signature.clone(),
            slot: signature.slot,
            relative_transaction_index: signature_index as u64,
            status: CrawlStatus::Pending,
            is_first_account_signature: is_last_batch && is_last_signature,
            error: None,
        };

        signatures_and_statuses.push((signature.signature.clone(), crawl_status));
    }

    Ok(signatures_and_statuses)
}
