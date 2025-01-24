use crate::pump_fun::coins::PumpFunCoin;
use borsh::{BorshDeserialize, BorshSchema};

#[derive(Debug)]
pub enum PumpFunInstruction {
    Create((CreateInstructionArgs, PumpFunCoin)),
    Buy((BuyInstructionArgs, PumpFunCoin)),
    Sell((SellInstructionArgs, PumpFunCoin)),
}

#[derive(BorshDeserialize, BorshSchema, Debug)]
pub struct CreateInstructionArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(BorshDeserialize, BorshSchema, Debug)]
pub struct BuyInstructionArgs {
    pub amount: u64,
    pub max_sol_cost: u64,
}

#[derive(BorshDeserialize, BorshSchema, Debug)]
pub struct SellInstructionArgs {
    pub amount: u64,
    pub min_sol_output: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum CreateAccountsOrder {
    Mint = 0,
    MintAuthority = 1,
    BondingCurve = 2,
    AssociatedBondingCurve = 3,
    Global = 4,
    MplTokenMetadata = 5,
    Metadata = 6,
    User = 7,
    SystemProgram = 8,
    TokenProgram = 9,
    AssociatedTokenProgram = 10,
    Rent = 11,
    EventAuthority = 12,
    Program = 13,
}

#[derive(Debug, Clone, Copy)]
pub enum BuyAccountsOrder {
    Global = 0,
    FeeRecipient = 1,
    Mint = 2,
    BondingCurve = 3,
    AssociatedBondingCurve = 4,
    AssociatedUser = 5,
    User = 6,
    SystemProgram = 7,
    TokenProgram = 8,
    Rent = 9,
    EventAuthority = 10,
    Program = 11,
}

#[derive(Debug, Clone, Copy)]
pub enum SellAccountsOrder {
    Global = 0,
    FeeRecipient = 1,
    Mint = 2,
    BondingCurve = 3,
    AssociatedBondingCurve = 4,
    AssociatedUser = 5,
    User = 6,
    SystemProgram = 7,
    AssociatedTokenProgram = 8,
    TokenProgram = 9,
    EventAuthority = 10,
    Program = 11,
}
