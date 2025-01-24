use crate::{
    client::{PUMP_FUN_PROGRAM_ID, RPC_CONCURRENCY, RPC_URL},
    pump_fun::coins::PumpFunCoin,
    utils::log::log_time,
    utils::pause::{pause, unpause, PauseSignal},
    utils::rate_limit::{check_rate_limit, set_rate_limit, RateLimitLock},
};
use crossbeam::channel::{Receiver, Sender};
use dashmap::DashSet;
use solana_client::{
    client_error::{reqwest::StatusCode, ClientError, ClientErrorKind},
    rpc_client::RpcClient,
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::{str::FromStr, sync::Arc, thread};

const CONCURRENCY: usize = RPC_CONCURRENCY;

#[derive(Debug)]
pub struct BondingCurveData {
    pub real_token_reserves: u64,
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
}

pub type BondingCurve = (Pubkey, BondingCurveData);

pub fn get_bonding_curve_datas(
    bonding_curves_tx: &Sender<BondingCurve>,
    pump_fun_coins_tx: &Sender<PumpFunCoin>,
    pump_fun_coins_rx: &Receiver<PumpFunCoin>,
    rate_limit_lock: &RateLimitLock,
    pause_signal: &PauseSignal,
) -> Vec<thread::JoinHandle<()>> {
    let mut handles = Vec::with_capacity(CONCURRENCY);

    let produced_curves = Arc::new(DashSet::new());

    let program_id = Pubkey::from_str(PUMP_FUN_PROGRAM_ID).unwrap();

    for i in 0..CONCURRENCY {
        let log_tag = format!("         {} bonding curves #{} | ", log_time(), i);

        let client =
            RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::confirmed());

        let pump_fun_coins_tx = pump_fun_coins_tx.clone();
        let pump_fun_coins_rx = pump_fun_coins_rx.clone();
        let bonding_curves_tx = bonding_curves_tx.clone();
        let produced_curves = produced_curves.clone();
        let rate_limit_lock = rate_limit_lock.clone();
        let pause_signal = pause_signal.clone();

        let handle = thread::spawn(move || {
            while let Ok((mint_address, bonding_curve_address)) = pump_fun_coins_rx.recv() {
                check_rate_limit(&rate_limit_lock);

                let (pda, _bump) = Pubkey::find_program_address(
                    &[b"bonding-curve", mint_address.as_ref()],
                    &program_id,
                );

                pause(&pause_signal);
                println!(
                    "{} Getting curve account data for mint {} pda {}",
                    log_tag, mint_address, pda
                );
                let account_data = client.get_account_data(&pda);
                unpause(&pause_signal);

                match account_data {
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
                    Err(err) => {
                        println!(
                            "{} Error getting curve account data.\n{:?}\nSkipping",
                            log_tag, err
                        );
                        continue;
                    }
                    Ok(data) => {
                        let curve = parse_bonding_curve_data(&data);
                        println!(
                            "{} Found curve account data for {} ------ {:?}",
                            log_tag, mint_address, curve
                        );

                        if produced_curves.insert(mint_address) {
                            bonding_curves_tx.send((mint_address, curve)).unwrap();
                        }
                    }
                };
            }
        });

        handles.push(handle);
    }

    handles
}

// TODO: Borsh decode?
fn parse_bonding_curve_data(data: &[u8]) -> BondingCurveData {
    if data.len() < 24 {
        panic!("Invalid bonding curve account data");
    }

    let mut real_token_bytes = [0u8; 8];
    real_token_bytes.copy_from_slice(&data[0..8]);
    let real_token_reserves = u64::from_le_bytes(real_token_bytes);

    let mut virtual_token_bytes = [0u8; 8];
    virtual_token_bytes.copy_from_slice(&data[8..16]);
    let virtual_token_reserves = u64::from_le_bytes(virtual_token_bytes);

    let mut virtual_sol_bytes = [0u8; 8];
    virtual_sol_bytes.copy_from_slice(&data[16..24]);
    let virtual_sol_reserves = u64::from_le_bytes(virtual_sol_bytes);

    BondingCurveData {
        real_token_reserves,
        virtual_token_reserves,
        virtual_sol_reserves,
    }
}

fn calculate_price(curve: &BondingCurveData, token_decimals: u8, amount: f64) -> f64 {
    let virtual_sol = curve.virtual_sol_reserves as f64;
    let virtual_tokens = curve.virtual_token_reserves as f64;

    let price_ratio = virtual_sol / virtual_tokens;

    let decimals_factor = 10f64.powi((9 - token_decimals) as i32);

    (price_ratio * amount) * decimals_factor
}
