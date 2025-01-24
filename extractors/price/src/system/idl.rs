use borsh::{BorshDeserialize, BorshSchema};

#[allow(dead_code)]
#[derive(BorshDeserialize, BorshSchema, Debug)]
pub struct SystemTransferInstructionData {
    pub lamports: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum SystemTransferAccountsOrder {
    Source = 0,
    Destination = 1,
}
