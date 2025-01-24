use crate::{
    client::{PUMP_FUN_PROGRAM_ID, RPC_CONCURRENCY, RPC_URL, TRANSACTION_CONFIG},
    pump_fun::{
        discriminators::build_pump_fun_instruction_discriminators, instructions::parse_instructions,
    },
    token::signatures::TokenSignature,
    transactions::is_failed_transaction,
    utils::{
        log::log_time,
        pause::{pause, unpause, PauseSignal},
        rate_limit::{check_rate_limit, set_rate_limit, RateLimitLock},
    },
};
use crossbeam::channel::{Receiver, Sender};
use dashmap::DashSet;
use solana_client::{
    client_error::{reqwest::StatusCode, ClientError, ClientErrorKind},
    rpc_client::RpcClient,
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signature};
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction, UiMessage,
};
use std::{str::FromStr, sync::Arc, thread};

const CONCURRENCY: usize = RPC_CONCURRENCY;

pub type TokenTransaction = (Pubkey, String, EncodedConfirmedTransactionWithStatusMeta);

pub fn get_token_transactions(
    token_transactions_tx: &Sender<TokenTransaction>,
    token_signatures_tx: &Sender<TokenSignature>,
    token_signatures_rx: &Receiver<TokenSignature>,
    rate_limit_lock: &RateLimitLock,
    pause_signal: &PauseSignal,
) -> Vec<thread::JoinHandle<()>> {
    let mut handles = Vec::with_capacity(CONCURRENCY);

    let pump_fun_program_id = Pubkey::from_str(PUMP_FUN_PROGRAM_ID).unwrap();

    let produced_transactions = Arc::new(DashSet::new());

    for i in 0..CONCURRENCY {
        let log_tag = format!("             {} token transactions #{} | ", log_time(), i);

        let token_transactions_tx = token_transactions_tx.clone();

        let client =
            RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::confirmed());

        let token_signatures_rx = token_signatures_rx.clone();
        let token_signatures_tx = token_signatures_tx.clone();
        let produced_transactions = produced_transactions.clone();
        let rate_limit_lock = rate_limit_lock.clone();
        let pause_signal = pause_signal.clone();
        let known_discriminators = build_pump_fun_instruction_discriminators();

        let handle = thread::spawn(move || {
            while let Ok((mint_address, token_tx_signature)) = token_signatures_rx.recv() {
                check_rate_limit(&rate_limit_lock);

                pause(&pause_signal);
                println!(
                    "{} Getting token transaction for {}",
                    log_tag, token_tx_signature
                );
                let tx = client.get_transaction_with_config(
                    &Signature::from_str(&token_tx_signature).unwrap(),
                    TRANSACTION_CONFIG,
                );
                unpause(&pause_signal);

                match tx {
                    Err(ClientError {
                        request: _,
                        kind: ClientErrorKind::Reqwest(ref reqwest_err),
                    }) => {
                        if let Some(status) = reqwest_err.status() {
                            if status == StatusCode::TOO_MANY_REQUESTS {
                                set_rate_limit(&rate_limit_lock, None);
                            }
                        }

                        println!("{} Rate limit exceeded. Retrying", log_tag);
                        token_signatures_tx
                            .send((mint_address, token_tx_signature))
                            .unwrap();
                    }
                    Err(error) => {
                        println!(
                            "{} Error getting token transaction.\n{:?}\nSkipping",
                            log_tag, error
                        );
                    }
                    Ok(tx) => {
                        if is_failed_transaction(&tx) {
                            println!("{} Token transaction failed. Skipping", log_tag);
                            continue;
                        }

                        println!(
                            "{} Got token transaction for {}",
                            log_tag, token_tx_signature
                        );

                        if let EncodedTransaction::Json(tx_json) = &tx.transaction.transaction {
                            if let UiMessage::Raw(raw_message) = &tx_json.message {
                                let account_keys = &raw_message.account_keys;

                                let parsed_instructions = parse_instructions(
                                    &known_discriminators,
                                    &account_keys,
                                    &raw_message,
                                    &pump_fun_program_id,
                                    &log_tag,
                                );

                                println!(
                                    "{} Pump fun coin tx parsed instructions: {:?}",
                                    log_tag, parsed_instructions
                                );

                                if account_keys.contains(&pump_fun_program_id.to_string())
                                    && produced_transactions.insert(token_tx_signature.clone())
                                {
                                    token_transactions_tx
                                        .send((mint_address, token_tx_signature, tx))
                                        .unwrap();
                                }
                            }
                        }
                    }
                }
            }
        });

        handles.push(handle);
    }

    handles
}
