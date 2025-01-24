use crate::{
    pump_fun::instructions::PumpFunInstruction, raydium::instructions::RaydiumInstruction,
};

pub type InstructionIndex = u64;

#[derive(Debug)]
pub enum Instruction {
    PumpFun(InstructionIndex, PumpFunInstruction),
    Raydium(InstructionIndex, RaydiumInstruction),
}
