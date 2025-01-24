use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub const RAYDIUM_AMM_ADDRESS: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";

pub fn get_raydium_amm_program_address() -> Pubkey {
    Pubkey::from_str(RAYDIUM_AMM_ADDRESS).unwrap()
}
