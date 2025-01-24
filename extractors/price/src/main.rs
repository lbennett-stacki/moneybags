use bonding_curve_data::get_bonding_curve_datas;
use client::IS_EXTRACTING_CURVE_DATA;
use db::client::{health_check, init_db};
use utils::pause::PauseSignal;
use pump_fun::coins::get_pump_fun_coins;
use pump_fun::signatures::get_pump_fun_signatures;
use utils::rate_limit::RateLimitLock;
use std::{
    error::Error,
    sync::{atomic::AtomicUsize, Arc, Mutex},
};
use token::prices::calc_token_prices;
use token::signatures::get_token_signatures;
use token::transactions::get_token_transactions;

mod anchor;
mod bonding_curve_data;
pub mod client;
mod db;
mod pump_fun;
mod utils;
mod raydium;
mod token;
mod transactions;

fn main() -> Result<(), Box<dyn Error>> {
    let rate_limit_lock: RateLimitLock = Arc::new(Mutex::new(None));
    let pause_signal: PauseSignal = Arc::new(AtomicUsize::new(0));

    let db = init_db();

    println!("WOOOO DB TEST --- -{:?}", health_check(&db));

    let (pump_fun_signatures_tx, pump_fun_signatures_rx) = crossbeam::channel::bounded(1000);
    let (pump_fun_coins_tx, pump_fun_coins_rx) = crossbeam::channel::bounded(10);
    let (token_signatures_tx, token_signatures_rx) = crossbeam::channel::unbounded();
    let (bonding_curves_tx, bonding_curves_rx) = crossbeam::channel::unbounded();
    let (token_transactions_tx, token_transactions_rx) = crossbeam::channel::unbounded();
    let (token_prices_tx, token_prices_rx) = crossbeam::channel::unbounded();

    let mut handles = Vec::new();

    let pump_fun_transaction_handles = get_pump_fun_signatures(
        pump_fun_signatures_tx.clone(),
        &rate_limit_lock,
        &pause_signal,
    );
    handles.extend(pump_fun_transaction_handles);

    let mint_address_handles = get_pump_fun_coins(
        &pump_fun_coins_tx,
        &pump_fun_signatures_tx,
        &pump_fun_signatures_rx,
        &rate_limit_lock,
        &pause_signal,
    );
    handles.extend(mint_address_handles);


    let (bonding_curves_pump_fun_coins_tx, bonding_curves_pump_fun_coins_rx) = crossbeam::channel::unbounded();
    let (token_signatures_pump_fun_coins_tx, token_signatures_pump_fun_coins_rx) = crossbeam::channel::unbounded();

    let bonding_tx = bonding_curves_pump_fun_coins_tx.clone();
    let token_sig_tx = token_signatures_pump_fun_coins_tx.clone();
    let inner_rx = pump_fun_coins_rx.clone();
    let coin_tee_handle = std::thread::spawn(move || {
        for pump_fun_coin in inner_rx {
            if IS_EXTRACTING_CURVE_DATA {
                bonding_tx.send(pump_fun_coin).unwrap();
            }
            token_sig_tx.send(pump_fun_coin).unwrap();
        }
    });
    handles.push(coin_tee_handle);


    let bonding_curves_handles = get_bonding_curve_datas(
        &bonding_curves_tx,
        &bonding_curves_pump_fun_coins_tx,
        &bonding_curves_pump_fun_coins_rx,
        &rate_limit_lock,
        &pause_signal,
    );
    handles.extend(bonding_curves_handles);

    let token_signatures_handles = get_token_signatures(
        &token_signatures_tx,
        &token_signatures_pump_fun_coins_tx,
        &token_signatures_pump_fun_coins_rx,
        &rate_limit_lock,
        &pause_signal,
    );
    handles.extend(token_signatures_handles);

    let token_transactions_handles = get_token_transactions(
        &token_transactions_tx,
        &token_signatures_tx,
        &token_signatures_rx,
        &rate_limit_lock,
        &pause_signal,
    );
    handles.extend(token_transactions_handles);

    let token_prices_handles =
        calc_token_prices(&token_prices_tx, &token_transactions_rx, &pause_signal);
    handles.extend(token_prices_handles);

    for (mint_address, _transaction, x, y) in token_prices_rx {
        println!(
            "                                       >>>>>>>     >>>>>>>     >>>>>>>    ------- Received token {} SOL price {} at block time {}",
            mint_address, y, x
        );
    }

    for (mint_address, curve_data) in bonding_curves_rx {
        println!(
                "                                       >>>>>>>     >>>>>>>     >>>>>>>    ------- Received token {} curve data {:?}",
            mint_address, curve_data 
        );
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
