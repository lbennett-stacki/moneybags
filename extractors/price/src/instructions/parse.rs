use crate::{
    cpi::cpi::CpiLog, instructions::instruction::Instruction,
    pump_fun::instructions::parse_single_pump_fun_instruction,
    raydium::instructions::parse_single_raydium_instruction,
};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug)]
pub struct InstructionWithLogs {
    pub instruction: Option<Instruction>,
    pub cpi_logs: Vec<CpiLog>,
}

#[derive(Debug, Clone)]
pub struct ParsableInstruction {
    pub data: String,
    pub accounts: Vec<u8>,
    pub program_id_index: usize,
    pub inner_instructions: Option<Vec<ParsableInstruction>>,
    pub instruction_index: usize,
    pub inner_instruction_index: Option<usize>,
}

pub fn parse_single_instruction(
    parsable: &ParsableInstruction,
    account_keys: &[String],
    instruction_index: u64,
    pump_fun_program_id: &Pubkey,
    raydium_amm_program_id: &Pubkey,
    inner_instructions: &Vec<ParsableInstruction>,
) -> Option<Instruction> {
    let exec = account_keys.get(parsable.program_id_index);
    if exec.is_none() {
        return None;
    }
    let executing_program_id_str = exec.unwrap();
    if executing_program_id_str == &pump_fun_program_id.to_string() {
        parse_single_pump_fun_instruction(&parsable, account_keys, instruction_index)
    } else if executing_program_id_str == &raydium_amm_program_id.to_string() {
        parse_single_raydium_instruction(
            &parsable,
            account_keys,
            instruction_index,
            inner_instructions,
        )
    } else {
        None
    }
}
