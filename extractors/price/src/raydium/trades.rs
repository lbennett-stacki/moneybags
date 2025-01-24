use super::{instructions::RaydiumInstruction, table::PoolAddress};
use crate::{
    instructions::instruction::InstructionIndex,
    trades::table::{TradeDirection, TradeRow},
};
use crossbeam::channel::Sender;
use time::OffsetDateTime;

pub fn get_trades_from_raydium_instruction(
    instruction: &RaydiumInstruction,
    instruction_index: &InstructionIndex,
    slot: u64,
    block_time: u64,
    token_tx_signature: &String,
    trades_tx: &Sender<TradeRow>,
    raydium_pools_tx: &Sender<PoolAddress>,
) {
    match instruction {
        RaydiumInstruction::SwapBaseIn((_, raydium_token, swap_amounts)) => {
            // TODO: dashset
            let (coin_token_address, pc_token_address) = raydium_token;
            let (coin_token_amount, pc_token_amount) = swap_amounts;
            raydium_pools_tx.send(*coin_token_address).unwrap();
            raydium_pools_tx.send(*pc_token_address).unwrap();
            trades_tx
                .send(TradeRow {
                    coin_token_address: coin_token_address.to_string(),
                    price_coin_token_address: pc_token_address.to_string(),
                    transaction_signature: token_tx_signature.clone(),
                    slot,
                    block_time: OffsetDateTime::from_unix_timestamp(block_time as i64).unwrap(),
                    instruction_index: *instruction_index,
                    coin_token_amount: *coin_token_amount,
                    price_coin_token_amount: *pc_token_amount,
                    direction: TradeDirection::Buy,
                })
                .unwrap();
        }
        RaydiumInstruction::SwapBaseOut((_, raydium_token, swap_amounts)) => {
            // TODO: dashset
            let (coin_token_address, pc_token_address) = raydium_token;
            let (coin_token_amount, pc_token_amount) = swap_amounts;
            raydium_pools_tx.send(*coin_token_address).unwrap();
            raydium_pools_tx.send(*pc_token_address).unwrap();
            trades_tx
                .send(TradeRow {
                    coin_token_address: coin_token_address.to_string(),
                    price_coin_token_address: pc_token_address.to_string(),
                    transaction_signature: token_tx_signature.clone(),
                    slot,
                    block_time: OffsetDateTime::from_unix_timestamp(block_time as i64).unwrap(),
                    instruction_index: *instruction_index,
                    coin_token_amount: *coin_token_amount,
                    price_coin_token_amount: *pc_token_amount,
                    direction: TradeDirection::Sell,
                })
                .unwrap();
        }
    };
}
