use crate::{
    client::{PUMP_FUN_PROGRAM_ID, RPC_URL, SIGNATURES_CONFIG},
    utils::{
        pause::{wait_for_unpause, PauseSignal},
        rate_limit::{check_rate_limit, set_rate_limit, RateLimitLock},
    },
};
use crossbeam::channel::Sender;
use dashmap::DashSet;
use solana_client::{
    client_error::{reqwest::StatusCode, ClientError, ClientErrorKind},
    rpc_client::RpcClient,
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::{str::FromStr, sync::Arc, thread};

pub fn get_pump_fun_signatures(
    pump_fun_signatures_tx: Sender<String>,
    rate_limit_lock: &RateLimitLock,
    pause_signal: &PauseSignal,
) -> Vec<thread::JoinHandle<()>> {
    let mut handles = Vec::with_capacity(1);

    let client = RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::confirmed());

    let produced_signatures = Arc::new(DashSet::new());

    let rate_limit_lock = rate_limit_lock.clone();
    let pause_signal = pause_signal.clone();

    let handle = thread::spawn(move || {
        let log_tag = "pump fun signatures | ";

        wait_for_unpause(&pause_signal);
        check_rate_limit(&rate_limit_lock);
        wait_for_unpause(&pause_signal);

        let program_id = Pubkey::from_str(PUMP_FUN_PROGRAM_ID).unwrap();

        println!("{} Getting pumpfun signatures", log_tag);
        let signatures =
            client.get_signatures_for_address_with_config(&program_id, SIGNATURES_CONFIG);

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
            }
            Err(error) => {
                println!(
                    "{} Error getting pumpfun signature.\n{:?}\nSkipping",
                    log_tag, error
                );
            }
            Ok(signatures) => {
                println!("{} Got pumpfun signatures ({})", log_tag, signatures.len());
                for signature in signatures {
                    let sig = signature.signature;

                    if produced_signatures.insert(sig.clone()) {
                        pump_fun_signatures_tx.send(sig).unwrap();
                        thread::sleep(std::time::Duration::from_secs(1));
                        wait_for_unpause(&pause_signal);
                    }
                }
            }
        };
    });

    handles.push(handle);

    handles
}
