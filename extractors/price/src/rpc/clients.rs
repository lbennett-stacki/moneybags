use super::pool::RpcClientState;
use crate::constants::IS_MAINNET;
use dotenvy_macro::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use std::collections::HashMap;

pub fn build_rpc_client(url: String) -> RpcClient {
    RpcClient::new_with_commitment(url, CommitmentConfig::confirmed())
}

pub fn build_rpc_client_states() -> HashMap<String, RpcClientState> {
    let mut map = HashMap::new();

    let states = if IS_MAINNET {
        vec![
            RpcClientState::new(dotenv!("PUBLIC_MAINNET_RPC_URL"), 1),
            // QUICKNODE_MAINNET_RPC_URL, -- Not currently supported
            RpcClientState::new(dotenv!("HELIUS_MAINNET_RPC_URL"), 1),
            RpcClientState::new(dotenv!("ALCHEMY_MAINNET_RPC_URL"), 1),
        ]
    } else {
        vec![
            RpcClientState::new(dotenv!("PUBLIC_DEVNET_RPC_URL"), 10),
            RpcClientState::new(dotenv!("QUICKNODE_DEVNET_RPC_URL"), 15),
            RpcClientState::new(dotenv!("HELIUS_DEVNET_RPC_URL"), 10),
            RpcClientState::new(dotenv!("ALCHEMY_DEVNET_RPC_URL"), 10),
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
