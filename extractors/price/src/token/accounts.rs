use crate::{
    constants::RPC_CONCURRENCY,
    pump_fun::tokens::PumpFunToken,
    rpc::pool::{RpcError, RpcPoolManager},
    token::table::TokenRow,
    utils::log::log_time,
};
use crossbeam::channel::{Receiver, Sender};
use solana_sdk::program_pack::Pack;
use std::thread;

const CONCURRENCY: usize = RPC_CONCURRENCY;

pub fn get_token_accounts(
    token_accounts_tx: &Sender<TokenRow>,
    pump_fun_tokens_rx: &Receiver<PumpFunToken>,
    rpc_pool_manager: &RpcPoolManager,
) -> Vec<thread::JoinHandle<()>> {
    let mut handles = Vec::with_capacity(CONCURRENCY);

    for thread_index in 0..CONCURRENCY {
        let log_tag = format!(
            "         {} token signatures #{} | ",
            log_time(),
            thread_index
        );

        let token_accounts_tx = token_accounts_tx.clone();
        let pump_fun_tokens_rx = pump_fun_tokens_rx.clone();
        let rpc_pool_manager = rpc_pool_manager.clone();

        let handle = thread::spawn(move || {
            while let Ok((mint_address, bonding_curve_address)) = pump_fun_tokens_rx.recv() {
                let mint_account = rpc_pool_manager.execute(
                    |client| client.get_account(&mint_address),
                    Some(thread_index as u64),
                );

                match mint_account {
                    Err(RpcError::ClientError(error)) => {
                        println!(
                            "{} Error getting mint account info.\n{:?}\nSkipping",
                            log_tag, error
                        );
                        continue;
                    }
                    Ok(account) => {
                        println!("{} Got mint account info for {}", log_tag, mint_address);

                        if let Ok(mint) = spl_token::state::Mint::unpack(&account.data) {
                            let token = TokenRow {
                                mint_address: mint_address.to_string(),
                                bonding_curve_address: bonding_curve_address.to_string(),
                                decimals: mint.decimals,
                            };

                            token_accounts_tx.send(token).unwrap();
                        } else {
                            println!("{} Failed to parse mint data. Skipping", log_tag);
                            continue;
                        }
                    }
                }
            }
        });

        handles.push(handle);
    }

    handles
}
