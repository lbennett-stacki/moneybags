use crate::{
    client::{PUMP_FUN_PROGRAM_ID, RPC_CONCURRENCY, RPC_URL, TRANSACTION_CONFIG},
    db::client::init_db,
    db::inserts::insert_token,
    db::tables::TokensRecord,
    pump_fun::{
        cpi::parse_transaction_logs, discriminators::build_pump_fun_instruction_discriminators,
        idl::PumpFunInstruction, instructions::parse_instructions,
    },
    transactions::is_failed_transaction,
    utils::{
        log::log_time,
        pause::{wait_for_unpause, PauseSignal},
        rate_limit::{check_rate_limit, set_rate_limit, RateLimitLock},
    },
};
use crossbeam::channel::{Receiver, Sender};
use dashmap::DashSet;
use solana_client::{
    client_error::{reqwest::StatusCode, ClientError, ClientErrorKind},
    rpc_client::RpcClient,
};
use solana_sdk::{
    commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey,
};
use solana_transaction_status::{EncodedTransaction, UiMessage};
use std::{str::FromStr, sync::Arc, thread};

const CONCURRENCY: usize = RPC_CONCURRENCY;

pub type PumpFunCoin = (/* mint */ Pubkey, /* bondingCurve */ Pubkey);

pub fn get_pump_fun_coins(
    pump_fun_coins_tx: &Sender<PumpFunCoin>,
    pump_fun_signatures_tx: &Sender<String>,
    pump_fun_signatures_rx: &Receiver<String>,
    rate_limit_lock: &RateLimitLock,
    pause_signal: &PauseSignal,
) -> Vec<thread::JoinHandle<()>> {
    let mut handles = Vec::with_capacity(CONCURRENCY);

    let pump_fun_program_id = Pubkey::from_str(PUMP_FUN_PROGRAM_ID).unwrap();

    let produced_addresses = Arc::new(DashSet::<String>::new());

    for i in 0..CONCURRENCY {
        let log_tag = format!("     {} mint addresses #{} | ", log_time(), i);

        let pump_fun_coins_tx = pump_fun_coins_tx.clone();

        let client =
            RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::confirmed());
        let db_client = init_db();

        let rate_limit_lock = rate_limit_lock.clone();
        let pause_signal = pause_signal.clone();
        let pump_fun_signatures_tx = pump_fun_signatures_tx.clone();
        let pump_fun_signatures_rx = pump_fun_signatures_rx.clone();
        let produced_addresses = produced_addresses.clone();
        let known_discriminators = build_pump_fun_instruction_discriminators();

        let handle = thread::spawn(move || {
            while let Ok(pump_fun_signature) = pump_fun_signatures_rx.recv() {
                wait_for_unpause(&pause_signal);
                check_rate_limit(&rate_limit_lock);
                wait_for_unpause(&pause_signal);

                if produced_addresses.contains(&pump_fun_signature) {
                    continue;
                }

                println!(
                    "\n     --------------------------------------------\n\n{} Getting mint addresses for {}",
                    log_tag, pump_fun_signature
                );
                let tx = client.get_transaction_with_config(
                    &pump_fun_signature.parse().unwrap(),
                    TRANSACTION_CONFIG,
                );

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
                        pump_fun_signatures_tx.send(pump_fun_signature).unwrap();
                        continue;
                    }
                    Err(error) => {
                        println!(
                            "{} Error getting mint address.\n{:?}\nSkipping",
                            log_tag, error
                        );
                        continue;
                    }
                    Ok(tx) => {
                        if is_failed_transaction(&tx) {
                            println!("{} Pump fun transaction failed. Skipping", log_tag);
                            continue;
                        }

                        println!("{} Got mint address tx", log_tag);

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

                                for parsed_instruction in parsed_instructions {
                                    if let Some(parsed_instruction) = parsed_instruction {
                                        println!(
                                            "{} Parsed instruction: {:?}",
                                            log_tag, parsed_instruction
                                        );
                                        let cpi_logs = parse_transaction_logs(&tx);

                                        if let Some(cpi_logs) = cpi_logs {
                                            for cpi_log in cpi_logs {
                                                if let Some(cpi_log) = cpi_log {
                                                    println!(
                                                        "{} Trade: {} SOL ({} lamports) for {} tokens",
                                                        log_tag,
                                                        cpi_log.sol_amount as f64
                                                            / LAMPORTS_PER_SOL as f64,
                                                        cpi_log.sol_amount,
                                                        cpi_log.token_amount as f64 / 1_000_000.0
                                                    );
                                                } else {
                                                    // failed to parse cpi log
                                                    continue;
                                                }
                                            }
                                        } else {
                                            // failed to parse cpi logs
                                            continue;
                                        }

                                        if produced_addresses.insert(pump_fun_signature.clone()) {
                                            let coin = match parsed_instruction {
                                                PumpFunInstruction::Create((_, coin)) => coin,
                                                PumpFunInstruction::Buy((_, coin)) => coin,
                                                PumpFunInstruction::Sell((_, coin)) => coin,
                                            };
                                            let (mint, bonding_curve) = coin;

                                            pump_fun_coins_tx.send(coin).unwrap();

                                            insert_token(
                                                &db_client,
                                                TokensRecord {
                                                    mint_address: mint.to_string(),
                                                    bonding_curve_address: bonding_curve
                                                        .to_string(),
                                                },
                                            );
                                            thread::sleep(std::time::Duration::from_secs(1));
                                            wait_for_unpause(&pause_signal);
                                        }
                                    } else {
                                        // failed to parse instruction
                                        continue;
                                    }
                                }
                            } else {
                                panic!("Unexpected pump fun tx message encoding");
                            }
                        } else {
                            panic!("Unexpected pump fun tx encoding");
                        }
                    }
                };
            }
        });

        handles.push(handle);
    }

    handles
}
