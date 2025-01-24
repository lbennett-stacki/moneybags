use solana_sdk::pubkey::Pubkey;

use super::idl::{SwapBaseInInstructionArgs, SwapBaseOutInstructionArgs};

pub type PoolCoinTokenAddress = Pubkey;
pub type PoolPcTokenAddress = Pubkey;
pub type RaydiumToken = (PoolCoinTokenAddress, PoolPcTokenAddress);

#[derive(Debug)]
pub enum RaydiumInstruction {
    SwapBaseIn((SwapBaseInInstructionArgs, RaydiumToken)),
    SwapBaseOut((SwapBaseOutInstructionArgs, RaydiumToken)),
}
