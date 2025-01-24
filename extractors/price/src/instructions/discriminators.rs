use crate::{
    instructions::instruction::InstructionDiscriminator,
    pump_fun::discriminators::build_pump_fun_instruction_discriminators,
    raydium::discriminators::build_raydium_instruction_discriminators,
};
use std::collections::HashMap;

pub fn build_instruction_discriminators() -> HashMap<[u8; 8], InstructionDiscriminator> {
    let mut map = HashMap::new();
    map.extend(
        build_pump_fun_instruction_discriminators()
            .iter()
            .map(|(k, v)| (*k, InstructionDiscriminator::PumpFun(*v))),
    );
    map.extend(
        build_raydium_instruction_discriminators()
            .iter()
            .map(|(k, v)| (*k, InstructionDiscriminator::Raydium(*v))),
    );
    map
}
