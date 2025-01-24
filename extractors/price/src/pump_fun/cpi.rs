use base64::{engine::general_purpose::STANDARD as base64, Engine as _};
use byteorder::{LittleEndian, ReadBytesExt};
use solana_sdk::pubkey::Pubkey;
use std::io::{Cursor, Read};

// TODO: use these features, remove allow dead
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct PumpFunCpiLog {
    pub sol_amount: u64,
    pub token_amount: u64,
    pub is_buy: bool,
    pub mint: Pubkey,
    pub user: Pubkey,
    pub timestamp: i64,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
}

impl PumpFunCpiLog {
    pub fn from_encoded_log(log: &str) -> Option<Self> {
        if !log.contains("Program data:") {
            return None;
        }

        let base64_data = log.split_whitespace().last()?;
        let decoded = base64.decode(base64_data).ok()?;
        let mut cursor = Cursor::new(decoded);

        cursor.set_position(8);

        let mint = {
            let mut mint_bytes = [0u8; 32];
            cursor.read_exact(&mut mint_bytes).ok()?;
            Pubkey::new_from_array(mint_bytes)
        };

        let sol_amount = cursor.read_u64::<LittleEndian>().ok()?;
        let token_amount = cursor.read_u64::<LittleEndian>().ok()?;
        let is_buy = cursor.read_u8().ok()? != 0;

        let user = {
            let mut user_bytes = [0u8; 32];
            cursor.read_exact(&mut user_bytes).ok()?;
            Pubkey::new_from_array(user_bytes)
        };

        let timestamp = cursor.read_i64::<LittleEndian>().ok()?;
        let virtual_sol_reserves = cursor.read_u64::<LittleEndian>().ok()?;
        let virtual_token_reserves = cursor.read_u64::<LittleEndian>().ok()?;

        Some(Self {
            sol_amount,
            token_amount,
            is_buy,
            mint,
            user,
            timestamp,
            virtual_sol_reserves,
            virtual_token_reserves,
        })
    }
}
