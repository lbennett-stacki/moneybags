use super::{
    idl::{BuyInstructionArgs, CreateInstructionArgs, SellInstructionArgs},
    tokens::PumpFunToken,
};

#[derive(Debug)]
pub enum PumpFunInstruction {
    Create((CreateInstructionArgs, PumpFunToken)),
    Buy((BuyInstructionArgs, PumpFunToken)),
    Sell((SellInstructionArgs, PumpFunToken)),
}
