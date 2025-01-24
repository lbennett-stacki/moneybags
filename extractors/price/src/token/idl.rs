use borsh::{BorshDeserialize, BorshSchema};

#[allow(dead_code)]
#[derive(BorshDeserialize, BorshSchema, Debug)]
pub struct TokenTransferInstructionData {
    pub amount: u64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum TokenTransferAccountsOrder {
    Source = 0,
    Destination = 1,
    Authority = 2,
}
