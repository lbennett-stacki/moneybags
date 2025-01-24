use crate::{
    pump_fun::{discriminators::PumpFunInstructionDiscriminator, instructions::PumpFunInstruction},
    raydium::{discriminators::RaydiumInstructionDiscriminator, instructions::RaydiumInstruction},
};

pub type InstructionIndex = u64;

#[derive(Debug)]
pub enum Instruction {
    PumpFun(InstructionIndex, PumpFunInstruction),
    Raydium(InstructionIndex, RaydiumInstruction),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum InstructionDiscriminator {
    PumpFun(PumpFunInstructionDiscriminator),
    Raydium(RaydiumInstructionDiscriminator),
}
