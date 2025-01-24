use super::{
    idl::{BuyInstructionArgs, CreateInstructionArgs, SellInstructionArgs},
    tokens::PumpFunToken,
};
use crate::{
    instructions::{instruction::Instruction, parse::ParsableInstruction},
    pump_fun::{
        discriminators::{
            build_pump_fun_instruction_discriminators, PumpFunInstructionDiscriminator,
        },
        idl::{BuyAccountsOrder, CreateAccountsOrder, SellAccountsOrder},
    },
};
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Debug)]
pub enum PumpFunInstruction {
    Create((CreateInstructionArgs, PumpFunToken)),
    Buy((BuyInstructionArgs, PumpFunToken)),
    Sell((SellInstructionArgs, PumpFunToken)),
}

pub fn parse_single_pump_fun_instruction(
    parsable: &ParsableInstruction,
    account_keys: &[String],
    instruction_index: u64,
) -> Option<Instruction> {
    let data = bs58::decode(&parsable.data).into_vec().ok()?;
    if data.len() < 8 {
        return None;
    }

    let discriminator: [u8; 8] = data[..8].try_into().unwrap();
    let pump_fun_discriminators = build_pump_fun_instruction_discriminators();

    let found_discriminator = pump_fun_discriminators.get(&discriminator);

    let instruction = match found_discriminator {
        None => None,
        Some(PumpFunInstructionDiscriminator::Create) => {
            let args = CreateInstructionArgs::deserialize(&mut &data[8..]).unwrap();

            // TODO: We now have found the name, symbol and metadata uri of a coin,
            // we should insert into db
            //and trigger a scrape of the metadata uri for json payload returning name, symbol, description, image uri, showName, createdOn, twitter, and maybe more?

            let mint = parsable
                .accounts
                .get(CreateAccountsOrder::Mint as usize)
                .unwrap();
            let bonding_curve = parsable
                .accounts
                .get(CreateAccountsOrder::BondingCurve as usize)
                .unwrap();
            let mint_str = account_keys.get(*mint as usize).unwrap();
            let bonding_curve_str = account_keys.get(*bonding_curve as usize).unwrap();
            let mint_address = Pubkey::from_str(mint_str).unwrap();
            let bonding_curve_address = Pubkey::from_str(bonding_curve_str).unwrap();

            Some(Instruction::PumpFun(
                instruction_index,
                PumpFunInstruction::Create((args, (mint_address, bonding_curve_address))),
            ))
        }
        Some(PumpFunInstructionDiscriminator::Buy) => {
            let args = BuyInstructionArgs::deserialize(&mut &data[8..]).unwrap();

            let mint: &u8 = parsable
                .accounts
                .get(BuyAccountsOrder::Mint as usize)
                .unwrap();
            let bonding_curve = parsable
                .accounts
                .get(BuyAccountsOrder::BondingCurve as usize)
                .unwrap();
            let mint_str = account_keys.get(*mint as usize).unwrap();
            let bonding_curve_str = account_keys.get(*bonding_curve as usize).unwrap();
            let mint_address = Pubkey::from_str(mint_str).unwrap();
            let bonding_curve_address = Pubkey::from_str(bonding_curve_str).unwrap();

            Some(Instruction::PumpFun(
                instruction_index,
                PumpFunInstruction::Buy((args, (mint_address, bonding_curve_address))),
            ))
        }
        Some(PumpFunInstructionDiscriminator::Sell) => {
            let args = SellInstructionArgs::deserialize(&mut &data[8..]).unwrap();

            let mint = parsable
                .accounts
                .get(SellAccountsOrder::Mint as usize)
                .unwrap();
            let bonding_curve = parsable
                .accounts
                .get(SellAccountsOrder::BondingCurve as usize)
                .unwrap();
            let mint_str = account_keys.get(*mint as usize).unwrap();
            let bonding_curve_str = account_keys.get(*bonding_curve as usize).unwrap();
            let mint_address = Pubkey::from_str(mint_str).unwrap();
            let bonding_curve_address = Pubkey::from_str(bonding_curve_str).unwrap();

            Some(Instruction::PumpFun(
                instruction_index,
                PumpFunInstruction::Sell((args, (mint_address, bonding_curve_address))),
            ))
        }
    };

    return instruction;
}
