use crate::{
    client::{RPC_CONCURRENCY, RPC_URL, SIGNATURES_CONFIG},
    pump_fun::coins::PumpFunCoin,
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
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::{sync::Arc, thread};

const CONCURRENCY: usize = RPC_CONCURRENCY;

pub type TokenSignature = (Pubkey, String);

pub fn get_token_signatures(
    token_signatures_tx: &Sender<TokenSignature>,
    pump_fun_coins_tx: &Sender<PumpFunCoin>,
    pump_fun_coins_rx: &Receiver<PumpFunCoin>,
    rate_limit_lock: &RateLimitLock,
    pause_signal: &PauseSignal,
) -> Vec<thread::JoinHandle<()>> {
    let mut handles = Vec::with_capacity(CONCURRENCY);

    let produced_signatures = Arc::new(DashSet::new());

    for i in 0..CONCURRENCY {
        let log_tag = format!("         {} token signatures #{} | ", log_time(), i);

        let token_signatures_tx = token_signatures_tx.clone();

        let client =
            RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::confirmed());

        let pump_fun_coins_rx = pump_fun_coins_rx.clone();
        let pump_fun_coins_tx = pump_fun_coins_tx.clone();
        let produced_signatures = produced_signatures.clone();
        let rate_limit_lock = rate_limit_lock.clone();
        let pause_signal = pause_signal.clone();

        let handle = thread::spawn(move || {
            while let Ok((mint_address, bonding_curve_address)) = pump_fun_coins_rx.recv() {
                check_rate_limit(&rate_limit_lock);

                pause(&pause_signal);
                println!("{} Getting token signatures for {}", log_tag, mint_address);
                let signatures =
                    client.get_signatures_for_address_with_config(&mint_address, SIGNATURES_CONFIG);
                unpause(&pause_signal);

                match signatures {
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
                        pump_fun_coins_tx
                            .send((mint_address, bonding_curve_address))
                            .unwrap();
                        continue;
                    }
                    Err(error) => {
                        println!(
                            "{} Error getting token signatures.\n{:?}\nSkipping",
                            log_tag, error
                        );
                        continue;
                    }
                    Ok(signatures) => {
                        println!(
                            "{} Got token signatures ({}) for {}",
                            log_tag,
                            signatures.len(),
                            mint_address
                        );
                        for signature in signatures {
                            let signature = signature.signature;
                            if produced_signatures.insert(signature.clone()) {
                                token_signatures_tx.send((mint_address, signature)).unwrap();
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
