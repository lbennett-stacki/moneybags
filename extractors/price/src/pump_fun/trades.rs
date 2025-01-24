use time::OffsetDateTime;

use crate::{
    cpi::cpi::CpiLog,
    instructions::instruction::InstructionIndex,
    system::program::SYSTEM_PROGRAM_ADDRESS,
    trades::{
        db::table::{TradeDirection, TradeRow},
        errors::TradeCrawlError,
    },
};

use super::instructions::PumpFunInstruction;

pub fn trade_from_pump_fun_instruction(
    instruction: &PumpFunInstruction,
    instruction_index: &InstructionIndex,
    slot: u64,
    block_time: u64,
    cpi_logs: &Vec<CpiLog>,
    token_tx_signature: &str,
) -> Result<Option<TradeRow>, TradeCrawlError> {
    let (discovered_mint_address, _bonding_curve_address) = match instruction {
        &PumpFunInstruction::Create((_, pump_fun_token)) => pump_fun_token,
        PumpFunInstruction::Buy((_, pump_fun_token)) => *pump_fun_token,
        PumpFunInstruction::Sell((_, pump_fun_token)) => *pump_fun_token,
    };
    let discovered_mint_address_string = discovered_mint_address.to_string();

    for cpi_log in cpi_logs.iter() {
        let CpiLog::PumpFun(cpi_log) = cpi_log;
        if cpi_log.mint.to_string() != discovered_mint_address_string {
            // log is not for this mint
            continue;
        }

        let block_time = OffsetDateTime::from_unix_timestamp(block_time as i64)
            .map_err(|error| TradeCrawlError::BlockTimeParseError(error))?;

        let trade = TradeRow {
            coin_token_address: discovered_mint_address_string.clone(),
            price_coin_token_address: SYSTEM_PROGRAM_ADDRESS.to_string(),
            transaction_signature: token_tx_signature.to_string(),
            slot,
            instruction_index: *instruction_index,
            block_time,
            coin_token_amount: cpi_log.token_amount,
            price_coin_token_amount: cpi_log.sol_amount,
            direction: if cpi_log.is_buy {
                TradeDirection::Buy
            } else {
                TradeDirection::Sell
            },
        };

        return Ok(Some(trade));
    }

    Ok(None)
}
