use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RaydiumInstructionDiscriminator {
    SwapBaseIn,
    SwapBaseOut,
}

pub fn build_raydium_instruction_discriminators() -> HashMap<u8, RaydiumInstructionDiscriminator> {
    let mut map = HashMap::new();

    map.insert(9, RaydiumInstructionDiscriminator::SwapBaseIn);
    map.insert(11, RaydiumInstructionDiscriminator::SwapBaseOut);

    map
}
