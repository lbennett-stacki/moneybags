use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub const PUMP_FUN_PROGRAM_ADDRESS: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";

pub fn get_pump_fun_program_address() -> Pubkey {
    Pubkey::from_str(PUMP_FUN_PROGRAM_ADDRESS).unwrap()
}
