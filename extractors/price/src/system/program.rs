use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub const SYSTEM_PROGRAM_ADDRESS: &str = "11111111111111111111111111111111";

pub fn get_system_program_address() -> Pubkey {
    Pubkey::from_str(SYSTEM_PROGRAM_ADDRESS).unwrap()
}
