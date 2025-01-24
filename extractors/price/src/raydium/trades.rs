use super::instructions::RaydiumInstruction;
use crate::{
    instructions::instruction::InstructionIndex,
    trades::db::table::{TradeDirection, TradeRow},
};
use time::OffsetDateTime;

pub fn trade_from_raydium_instruction(
    instruction: &RaydiumInstruction,
    instruction_index: &InstructionIndex,
    slot: u64,
    block_time: u64,
    token_tx_signature: &str,
) -> TradeRow {
    let mut raydium_pools = Vec::new();

    match instruction {
        RaydiumInstruction::SwapBaseIn((_, raydium_token, swap_amounts)) => {
            let (coin_token_address, pc_token_address) = raydium_token;
            let (coin_token_amount, pc_token_amount) = swap_amounts;

            TradeRow {
                coin_token_address: coin_token_address.to_string(),
                price_coin_token_address: pc_token_address.to_string(),
                transaction_signature: token_tx_signature.to_string(),
                slot,
                block_time: OffsetDateTime::from_unix_timestamp(block_time as i64).unwrap(),
                instruction_index: *instruction_index,
                coin_token_amount: *coin_token_amount,
                price_coin_token_amount: *pc_token_amount,
                direction: TradeDirection::Buy,
            }
        }
        RaydiumInstruction::SwapBaseOut((_, raydium_token, swap_amounts)) => {
            let (coin_token_address, pc_token_address) = raydium_token;
            let (coin_token_amount, pc_token_amount) = swap_amounts;

            raydium_pools.push((coin_token_address, pc_token_address));

            TradeRow {
                coin_token_address: coin_token_address.to_string(),
                price_coin_token_address: pc_token_address.to_string(),
                transaction_signature: token_tx_signature.to_string(),
                slot,
                block_time: OffsetDateTime::from_unix_timestamp(block_time as i64).unwrap(),
                instruction_index: *instruction_index,
                coin_token_amount: *coin_token_amount,
                price_coin_token_amount: *pc_token_amount,
                direction: TradeDirection::Sell,
            }
        }
    }
}
