use borsh::{BorshDeserialize, BorshSchema};

#[allow(dead_code)]
#[derive(BorshDeserialize, BorshSchema, Debug)]
pub struct SwapBaseInInstructionArgs {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

#[allow(dead_code)]
#[derive(BorshDeserialize, BorshSchema, Debug)]
pub struct SwapBaseOutInstructionArgs {
    pub max_amount_in: u64,
    pub amount_out: u64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum SwapBaseInAccountsOrder {
    TokenProgram = 0,
    Amm = 1,
    AmmAuthority = 2,
    AmmOpenOrders = 3,
    AmmTargetOrders = 4,
    PoolCoinTokenAccount = 5,
    PoolPcTokenAccount = 6,
    SerumProgram = 7,
    SerumMarket = 8,
    SerumBids = 9,
    SerumAsks = 10,
    SerumEventQueue = 11,
    SerumCoinVaultAccount = 12,
    SerumPcVaultAccount = 13,
    SerumVaultSigner = 14,
    UserSourceTokenAccount = 15,
    UserDestinationTokenAccount = 16,
    UserSourceOwner = 17,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum SwapBaseOutAccountsOrder {
    TokenProgram = 0,
    Amm = 1,
    AmmAuthority = 2,
    AmmOpenOrders = 3,
    AmmTargetOrders = 4,
    PoolCoinTokenAccount = 5,
    PoolPcTokenAccount = 6,
    SerumProgram = 7,
    SerumMarket = 8,
    SerumBids = 9,
    SerumAsks = 10,
    SerumEventQueue = 11,
    SerumCoinVaultAccount = 12,
    SerumPcVaultAccount = 13,
    SerumVaultSigner = 14,
    UserSourceTokenAccount = 15,
    UserDestinationTokenAccount = 16,
    UserSourceOwner = 17,
}
