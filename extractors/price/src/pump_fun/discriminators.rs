use std::collections::HashMap;

use crate::anchor::hash_anchor_discriminator;

#[derive(PartialEq, Debug)]
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
