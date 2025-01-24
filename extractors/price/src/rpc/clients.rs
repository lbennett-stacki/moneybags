use std::collections::HashMap;

use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

use crate::constants::IS_MAINNET;

use super::pool::RpcClientState;

pub fn build_rpc_client(url: String) -> RpcClient {
    RpcClient::new_with_commitment(url, CommitmentConfig::confirmed())
}

pub fn build_rpc_client_states() -> HashMap<String, RpcClientState> {
    let mut map = HashMap::new();

    let states = if IS_MAINNET {
        vec![
            RpcClientState::new(std::env::var("PUBLIC_MAINNET_RPC_URL").unwrap(), 10),
            // QUICKNODE_MAINNET_RPC_URL, -- Not currently supported
            RpcClientState::new(std::env::var("HELIUS_MAINNET_RPC_URL").unwrap(), 10),
            RpcClientState::new(std::env::var("ALCHEMY_MAINNET_RPC_URL").unwrap(), 10),
        ]
    } else {
        vec![
            RpcClientState::new(std::env::var("PUBLIC_DEVNET_RPC_URL").unwrap(), 10),
            RpcClientState::new(std::env::var("QUICKNODE_DEVNET_RPC_URL").unwrap(), 15),
            RpcClientState::new(std::env::var("HELIUS_DEVNET_RPC_URL").unwrap(), 10),
            RpcClientState::new(std::env::var("ALCHEMY_DEVNET_RPC_URL").unwrap(), 10),
        ]
    };

    for state in states {
        map.insert(state.url.clone(), state);
    }

    map
}

pub fn get_rpc_nodes_count() -> usize {
    build_rpc_client_states().len()
}
