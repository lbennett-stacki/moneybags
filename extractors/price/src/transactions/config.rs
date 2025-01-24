use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::UiTransactionEncoding;

pub const TRANSACTION_CONFIG: RpcTransactionConfig = RpcTransactionConfig {
    encoding: Some(UiTransactionEncoding::Json),
    commitment: Some(CommitmentConfig::confirmed()),
    max_supported_transaction_version: Some(0),
};
