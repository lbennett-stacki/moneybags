use crate::{
    client::CALC_CONCURRENCY,
    token::transactions::TokenTransaction,
    utils::{
        log::log_time,
        pause::{pause, unpause, PauseSignal},
    },
};
use crossbeam::channel::{Receiver, Sender};
use dashmap::DashSet;
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction, UiMessage,
};
use std::{sync::Arc, thread};

const CONCURRENCY: usize = CALC_CONCURRENCY;

pub type TokenPrice = (Pubkey, EncodedConfirmedTransactionWithStatusMeta, i64, f64);

pub fn calc_token_prices(
    token_prices_tx: &Sender<TokenPrice>,
    token_transactions_rx: &Receiver<TokenTransaction>,
    pause_signal: &PauseSignal,
) -> Vec<thread::JoinHandle<()>> {
    let mut handles = Vec::with_capacity(CONCURRENCY);

    let produced_prices = Arc::new(DashSet::new());

    for i in 0..CONCURRENCY {
        let log_tag = format!("                 {} token prices #{} | ", log_time(), i);

        let mint_addresses_rx = token_transactions_rx.clone();
        let token_prices_tx = token_prices_tx.clone();
        let produced_prices = produced_prices.clone();
        let pause_signal = pause_signal.clone();

        let handle = thread::spawn(move || {
            while let Ok((mint_address, token_tx_signature, transaction)) = mint_addresses_rx.recv()
            {
                pause(&pause_signal);
                println!("{} Calculating price for {}", log_tag, token_tx_signature);

                let k = 20000.0; // TODO: Adjust based on actual bonding curve

                let mut supply = 0.0;
                let mut reserve = 0.0;

                let block_time = transaction.block_time.unwrap_or(0);
                let delta_sol = parse_sol_transfer(&transaction);

                reserve += delta_sol;
                let new_supply = ((reserve * 3.0 * k).powf(1.0 / 3.0)).round();
                let delta_supply = new_supply - supply;

                if delta_supply > 0.0 {
                    supply = new_supply;
                    let price = (supply * supply) / k;

                    if produced_prices.insert(token_tx_signature) {
                        token_prices_tx
                            .send((mint_address, transaction, block_time, price))
                            .unwrap();
                    }
                }

                unpause(&pause_signal);
            }
        });

        handles.push(handle);
    }

    handles
}

fn parse_sol_transfer(
    tx: &solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta,
) -> f64 {
    let mut all_sol = Vec::new();
    if let EncodedTransaction::Json(tx_json) = &tx.transaction.transaction {
        if let UiMessage::Raw(raw_message) = &tx_json.message {
            let account_keys = &raw_message.account_keys;

            for ix in &raw_message.instructions {
                let program_id_index = ix.program_id_index as usize;
                let actual_program_id_str = account_keys.get(program_id_index);

                if actual_program_id_str.is_some()
                    && *actual_program_id_str.unwrap()
                        != solana_sdk::system_program::id().to_string()
                {
                    continue;
                }

                if ix.data.len() < 12 {
                    continue;
                }

                let lamports = ix.data.as_bytes();
                let lamports: [u8; 8] = lamports[4..12].try_into().unwrap();
                let lamports = u64::from_le_bytes(lamports);
                let sol = lamports as f64 / 1_000_000_000.0;
                all_sol.push(sol);
            }
        }
    }

    all_sol.iter().sum()
}
