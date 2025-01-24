use clap::Parser;
use crawl_status::store::store_crawl_statuses;
use crawl_status::table::{CrawlStatus, CrawlStatusOperation, CrawlStatusRow};
use db::client::db_health_check;
use db::init::init_db;
use dotenvy::dotenv;
use dragonfly::client::dragonfly_client;
use dragonfly::health::dragonfly_health_check;
use pump_fun::program::signatures::TransactionSignature;
use pump_fun::program::signatures_threads::pump_fun_program_signatures_threads;
use pump_fun::tokens::PumpFunToken;
use pump_fun::tokens_threads::pump_fun_tokens_threads;
use rpc::pool::{RpcPoolManager, DEFAULT_RATE_LIMIT_COOLOFF_MS};
use solana_sdk::pubkey::Pubkey;
use std::error::Error;
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use termination::init as termination_init;
use token::accounts::get_token_accounts_meta;
use token::mint::signatures::{get_token_mint_signatures, TokenMintSignatures};
use token::store::store_tokens;
use token::table::TokenRow;
use trades::db::store::store_trades;
use trades::db::table::TradeRow;
use trades::trades_threads::token_trades_threads;
use utils::blocking::blocking_call;

mod anchor;
mod constants;
mod cpi;
mod crawl_status;
mod db;
mod dragonfly;
mod instructions;
mod pump_fun;
mod raydium;
mod rpc;
mod signatures;
mod system;
mod termination;
mod token;
mod trades;
mod transactions;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    token: Option<String>,

    #[arg(long)]
    tx: Option<String>,
}

#[cfg(test)]
mod tests;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;
    let args = Args::parse();

    let targetted_mint_address = args.token;
    let target_transaction_signature = args.tx;

    if target_transaction_signature.is_some() && targetted_mint_address.is_none() {
        panic!("Cannot set target transaction signature without setting target token mint address");
    }

    let termination_flag = termination_init();

    let rpc_pool_manager =
        RpcPoolManager::new(Duration::from_millis(DEFAULT_RATE_LIMIT_COOLOFF_MS));

    let db = blocking_call(async { init_db().await });
    let dragonfly = dragonfly_client();

    println!("DB health check: {:?}", db_health_check(&db));
    println!(
        "Dragonfly health check: {:?}",
        dragonfly_health_check(&dragonfly)
    );

    let (pump_fun_program_signatures_tx, pump_fun_program_signatures_rx) =
        crossbeam::channel::bounded::<TransactionSignature>(1);
    let (pump_fun_tokens_tx, pump_fun_tokens_rx) = crossbeam::channel::bounded::<PumpFunToken>(1);
    let (token_accounts_tx, token_accounts_rx) = crossbeam::channel::unbounded::<TokenRow>();
    let (token_pump_fun_signatures_tx, token_pump_fun_signatures_rx) =
        crossbeam::channel::unbounded::<TokenMintSignatures>();
    let (pump_fun_trades_tx, pump_fun_trades_rx) = crossbeam::channel::unbounded::<TradeRow>();
    let (crawl_status_tx, crawl_status_rx) =
        crossbeam::channel::unbounded::<CrawlStatusOperation>();

    //
    //                                      ┌────────────────────────────────────────┐
    //                                      │ get recent pump fun program signatures │
    //                                      └────────────────────────────────────────┘
    //                                                          ┴
    //                                             pump_fun_program_signatures
    //                                                          ┬
    //                                         ┌─────────────────────────────────┐
    //                                         │ decode transactions and extract │      ┌────────────────────┐
    //       ┌─────────────────────────────────│ mint and bonding curve address  │──────│ store crawl status │
    //       ┴                                 └─────────────────────────────────┘      └────────────────────┘
    // if buy/sell instruction,                       ┴                     ┴
    // token_pump_fun_transactions             pump_fun_tokens         raydium_pool
    //       ┬                                        ┬                     ┬
    //       │          ┌────────────────────┬────────┴──────────────┐      └──────────────────────┐
    //       │  ┌───────────────┐ ┌─────────────────────┐ ┌──────────────────────┐ ┌───────────────────────┐
    //       │  │ get mint data │ │ get mint signatures │ │ get curve signatures │ │ get raydium pool data │
    //       │  └───────────────┘ └─────────────────────┘ └──────────────────────┘ └───────────────────────┘
    //       │          ┴                    └──────────────────┬────┴───────────────┐
    //       │    token_accounts                                ┴                    │
    //       │          ┬                           token_pump_fun_signatures        │ ┌────────────────────┐
    //       │  ┌─────────────────┐                             ┬                    └─│ store crawl status │
    //       │  │ store acc data  │                   ┌─────────┴──────────────────┐   └────────────────────┘
    //       │  │  decimals, etc  │ ┌-───────────────────────────────────┐ ┌────────────────┐
    //       │  └─────────────────┘ │   decode instructions and extract  │ │ get block data │
    //       │                      │ instruction meta and token amounts │ └────────────────┘
    //       │                      └────────────────────────────────────┘          ┴
    //       │                                                  ┴                 blocks
    //       │                                                trades                ┬
    //       │                                                  ┬        ┌────────────────────────┐
    //       │                                           ┌─────────────┐ │      store block       │
    //       └───────────────────────────────────────────│ store trade │ │     tx indexes etc     │
    //                                                   └─────────────┘ └────────────────────────┘
    //
    //
    //

    let mut handles = Vec::new();

    if targetted_mint_address.is_none() && target_transaction_signature.is_none() {
        let handle = pump_fun_program_signatures_threads(
            &pump_fun_program_signatures_tx,
            &crawl_status_tx,
            &rpc_pool_manager,
            &termination_flag,
        );
        handles.extend(handle);
    } else {
        println!(
            "User has set a target mint address. Skipping extracting pump fun program signatures"
        );
    };

    if target_transaction_signature.is_some() {
        println!("User has set a target transaction signature. Skipping mint address crawls.");
    } else if let Some(targetted_mint_address) = targetted_mint_address.clone() {
        println!("User has set a target mint address. Sending single mint address for crawl.");
        let target = targetted_mint_address;
        pump_fun_tokens_tx
            .send((
                Pubkey::from_str(&target).unwrap(),
                // TODO: this one should be conding curve address
                Pubkey::from_str(&target).unwrap(),
            ))
            .unwrap();
    } else {
        let handle = pump_fun_tokens_threads(
            &pump_fun_tokens_tx,
            &pump_fun_trades_tx,
            &pump_fun_program_signatures_rx,
            &crawl_status_tx,
            &rpc_pool_manager,
        );

        handles.extend(handle);
    }

    let (pump_fun_tokens_tx_token_account_tee, pump_fun_tokens_rx_token_account_tee) =
        crossbeam::channel::unbounded();
    let (pump_fun_tokens_tx_mint_signatures_tee, pump_fun_tokens_rx_mint_signatures_tee) =
        crossbeam::channel::unbounded();
    let pump_fun_tokens_tx_mint_signatures_tee_inner =
        pump_fun_tokens_tx_mint_signatures_tee.clone();
    let tee_handles = thread::spawn(move || {
        for token in pump_fun_tokens_rx {
            pump_fun_tokens_tx_token_account_tee.send(token).unwrap();
            pump_fun_tokens_tx_mint_signatures_tee_inner
                .send(token)
                .unwrap();
        }
    });
    handles.push(tee_handles);

    let token_accounts_handles = get_token_accounts_meta(
        &token_accounts_tx,
        &pump_fun_tokens_rx_token_account_tee,
        &rpc_pool_manager,
    );
    handles.extend(token_accounts_handles);

    let token_mint_signatures_handles = get_token_mint_signatures(
        &token_pump_fun_signatures_tx,
        &crawl_status_tx,
        &pump_fun_tokens_rx_mint_signatures_tee,
        &pump_fun_tokens_tx_mint_signatures_tee,
        &rpc_pool_manager,
    );
    handles.extend(token_mint_signatures_handles);

    if let Some(target_transaction_signature) = target_transaction_signature {
        println!("User has set a target transaction signature. Sending single transaction signature for crawl.");
        let mint_address = targetted_mint_address.unwrap();

        token_pump_fun_signatures_tx
            .send((
                Pubkey::from_str(&mint_address.clone()).unwrap(),
                target_transaction_signature.clone(),
            ))
            .unwrap();

        crawl_status_tx
            .send(CrawlStatusOperation::Create(CrawlStatusRow {
                account_address: mint_address,
                transaction_signature: target_transaction_signature.clone(),
                slot: 0,
                relative_transaction_index: 0,
                is_first_account_signature: false,
                status: CrawlStatus::Pending,
                error: None,
            }))
            .unwrap();
    }

    let token_trades_handles = token_trades_threads(
        &pump_fun_trades_tx,
        &token_pump_fun_signatures_rx,
        &rpc_pool_manager,
        &crawl_status_tx,
        &termination_flag,
    );
    handles.extend(token_trades_handles);

    let token_prices_handles = store_trades(&pump_fun_trades_rx);
    handles.extend(token_prices_handles);

    let store_tokens_handles = store_tokens(&token_accounts_rx);
    handles.extend(store_tokens_handles);

    let store_crawl_statuses_handles = store_crawl_statuses(&crawl_status_rx);
    handles.extend(store_crawl_statuses_handles);

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
