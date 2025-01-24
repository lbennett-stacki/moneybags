use crate::anchor::hash_anchor_discriminator;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RaydiumInstructionDiscriminator {
    SwapBaseIn,
    SwapBaseOut,
}

pub fn build_raydium_instruction_discriminators(
) -> HashMap<[u8; 8], RaydiumInstructionDiscriminator> {
    let mut map = HashMap::new();
    map.insert(
        hash_anchor_discriminator("swapBaseIn"),
        RaydiumInstructionDiscriminator::SwapBaseIn,
    );
    map.insert(
        hash_anchor_discriminator("swapBaseOut"),
        RaydiumInstructionDiscriminator::SwapBaseOut,
    );
    map.insert(
        hash_anchor_discriminator("swap"),
        RaydiumInstructionDiscriminator::SwapBaseIn,
    );
    map.insert(
        hash_anchor_discriminator("raydium:swap"),
        RaydiumInstructionDiscriminator::SwapBaseIn,
    );
    map.insert(
        hash_anchor_discriminator("raydium_swap"),
        RaydiumInstructionDiscriminator::SwapBaseIn,
    );
    map
}
