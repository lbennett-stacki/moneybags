use crate::anchor::hash::hash_anchor_discriminator;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum PumpFunInstructionDiscriminator {
    Buy,
    Sell,
    Create,
}

pub fn build_pump_fun_instruction_discriminators(
) -> HashMap<[u8; 8], PumpFunInstructionDiscriminator> {
    let mut map = HashMap::new();
    map.insert(
        hash_anchor_discriminator("buy"),
        PumpFunInstructionDiscriminator::Buy,
    );
    map.insert(
        hash_anchor_discriminator("sell"),
        PumpFunInstructionDiscriminator::Sell,
    );
    map.insert(
        hash_anchor_discriminator("create"),
        PumpFunInstructionDiscriminator::Create,
    );
    map
}
