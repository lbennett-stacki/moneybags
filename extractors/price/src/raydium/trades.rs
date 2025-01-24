use super::{instructions::RaydiumInstruction, table::PoolAddress};
use crate::{
    cpi::cpi::CpiLog, instructions::instruction::InstructionIndex, trades::table::TradeRow,
};
use crossbeam::channel::Sender;

pub fn get_trades_from_raydium_instruction(
    instruction: &RaydiumInstruction,
    instruction_index: &InstructionIndex,
    slot: u64,
    block_time: u64,
    instruction_cpi_logs: &Vec<CpiLog>,
    token_tx_signature: &String,
    trades_tx: &Sender<TradeRow>,
    raydium_pools_tx: &Sender<PoolAddress>,
) {
    let (coin_token_address, price_coin_token_address) = match instruction {
        RaydiumInstruction::SwapBaseIn((_, raydium_token)) => {
            // TODO: dashset
            let (coin_token_address, pc_token_address) = raydium_token;
            raydium_pools_tx.send(*coin_token_address).unwrap();
            raydium_pools_tx.send(*pc_token_address).unwrap();
            raydium_token
        }
        RaydiumInstruction::SwapBaseOut((_, raydium_token)) => {
            // TODO: dashset
            let (coin_token_address, pc_token_address) = raydium_token;
            raydium_pools_tx.send(*coin_token_address).unwrap();
            raydium_pools_tx.send(*pc_token_address).unwrap();
            raydium_token
        }
    };

    for cpi_log in instruction_cpi_logs.iter() {
        if let CpiLog::Raydium(cpi_log) = cpi_log {
            // TODO: do we need a check similar to this one that we use in pump fun trades instruction parsing?
            // if cpi_log.mint.to_string() != discovered_mint_address_string {
            //     // log is not for this mint
            //     continue;
            // }

            println!("Found a cpi log for raydium -- {:#?}", cpi_log);
            // TODO: temp skipping trade row for development
            continue;

            // let trade = TradeRow {
            //     coin_token_address: coin_token_address.to_string(),
            //     price_coin_token_address: price_coin_token_address.to_string(),
            //     transaction_signature: token_tx_signature.clone(),
            //     slot,
            //     instruction_index: *instruction_index,
            //     block_time: OffsetDateTime::from_unix_timestamp(block_time as i64).unwrap(),
            //     // coin_token_amount: cpi_log.coin_token_amount,
            //     // price_coin_token_amount: cpi_log.price_coin_amount,
            //     // direction: if cpi_log.is_buy {
            //     //     TradeDirection::Buy
            //     // } else {
            //     //     TradeDirection::Sell
            //     // },
            // };

            // trades_tx.send(trade).unwrap();
        }
    }
}
