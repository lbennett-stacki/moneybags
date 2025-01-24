use solana_client::{
    rpc_client::GetConfirmedSignaturesForAddress2Config, rpc_config::RpcTransactionConfig,
};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::UiTransactionEncoding;

pub const IS_MAINNET: bool = false;
pub const IS_ONE_TOKEN_AT_A_TIME: bool = true;
pub const IS_EXTRACTING_CURVE_DATA: bool = false;

pub const RPC_URL: &str = if IS_MAINNET {
    "https://api.mainnet-beta.solana.com"
} else {
    "https://api.devnet.solana.com"
};

pub const PUMP_FUN_PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";

pub const RPC_CONCURRENCY: usize = if IS_MAINNET { 2 } else { 4 };
pub const CALC_CONCURRENCY: usize = 2;

pub const TRANSACTION_CONFIG: RpcTransactionConfig = RpcTransactionConfig {
    encoding: Some(UiTransactionEncoding::Json),
    commitment: Some(CommitmentConfig::confirmed()),
    max_supported_transaction_version: Some(0),
};

pub const SIGNATURES_CONFIG: GetConfirmedSignaturesForAddress2Config =
    GetConfirmedSignaturesForAddress2Config {
        commitment: Some(CommitmentConfig::confirmed()),
        before: None,
        until: None,
        limit: Some(1000),
    };

pub const CLICKHOUSE_URL: &str = "http://localhost:8123";
pub const CLICKHOUSE_DB_NAME: &str = "moneybags";

pub const WAIT_FOR_UNPAUSE_MS: u64 = 500;
