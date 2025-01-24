use super::{
    discriminators::PumpFunInstructionDiscriminator,
    idl::{
        BuyAccountsOrder, BuyInstructionArgs, CreateAccountsOrder, CreateInstructionArgs,
        PumpFunInstruction, SellAccountsOrder, SellInstructionArgs,
    },
};
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::UiRawMessage;
use std::{collections::HashMap, str::FromStr};

pub fn parse_instructions(
    known_discriminators: &HashMap<[u8; 8], PumpFunInstructionDiscriminator>,
    account_keys: &[String],
    raw_message: &UiRawMessage,
    pump_fun_program_id: &Pubkey,
    log_tag: &str,
) -> Vec<Option<PumpFunInstruction>> {
    let log_tag = format!("{} ~ Parsing instructions | ", log_tag);

    raw_message
        .instructions
        .iter()
        .map(|ix| {
            let executing_program_id_index = ix.program_id_index as usize;
            if let Some(executing_program_id_str) = account_keys.get(executing_program_id_index) {
                if executing_program_id_str != &pump_fun_program_id.to_string() {
                    // is not executing on the pump fun program
                    return None;
                }

                let data = bs58::decode(&ix.data).into_vec().unwrap();
                if data.len() < 8 {
                    // instruction data too short
                    return None;
                }

                let discriminator: [u8; 8] = data[..8].try_into().unwrap();

                let parsed_instruction = match known_discriminators.get(&discriminator) {
                    Some(instruction) => match instruction {
                        PumpFunInstructionDiscriminator::Create => {
                            let args = CreateInstructionArgs::try_from_slice(&data[8..]).unwrap();

                            // TODO: We now have found the name, symbol and metadata uri of a coin,
                            // we should insert into db
                            //and trigger a scrape of the metadata uri for json payload returning name, symbol, description, image uri, showName, createdOn, twitter, and maybe more?

                            let mint = ix.accounts.get(CreateAccountsOrder::Mint as usize).unwrap();
                            let bonding_curve = ix
                                .accounts
                                .get(CreateAccountsOrder::BondingCurve as usize)
                                .unwrap();
                            let mint_str = account_keys.get(*mint as usize).unwrap();
                            let bonding_curve_str =
                                account_keys.get(*bonding_curve as usize).unwrap();
                            let mint_address = Pubkey::from_str(mint_str).unwrap();
                            let bonding_curve_address =
                                Pubkey::from_str(bonding_curve_str).unwrap();

                            Some(PumpFunInstruction::Create((
                                args,
                                (mint_address, bonding_curve_address),
                            )))
                        }

                        // TODO: these buy and sell branches are a valued transcation in themselves, so we should insert into a tx table to avoid re-reading/parsing etc
                        PumpFunInstructionDiscriminator::Buy => {
                            let args = BuyInstructionArgs::try_from_slice(&data[8..]).unwrap();

                            let mint = ix.accounts.get(BuyAccountsOrder::Mint as usize).unwrap();
                            let bonding_curve = ix
                                .accounts
                                .get(BuyAccountsOrder::BondingCurve as usize)
                                .unwrap();
                            let mint_str = account_keys.get(*mint as usize).unwrap();
                            let bonding_curve_str =
                                account_keys.get(*bonding_curve as usize).unwrap();
                            let mint_address = Pubkey::from_str(mint_str).unwrap();
                            let bonding_curve_address =
                                Pubkey::from_str(bonding_curve_str).unwrap();

                            Some(PumpFunInstruction::Buy((
                                args,
                                (mint_address, bonding_curve_address),
                            )))
                        }
                        PumpFunInstructionDiscriminator::Sell => {
                            let args = SellInstructionArgs::try_from_slice(&data[8..]).unwrap();

                            let mint = ix.accounts.get(SellAccountsOrder::Mint as usize).unwrap();
                            let bonding_curve = ix
                                .accounts
                                .get(SellAccountsOrder::BondingCurve as usize)
                                .unwrap();
                            let mint_str = account_keys.get(*mint as usize).unwrap();
                            let bonding_curve_str =
                                account_keys.get(*bonding_curve as usize).unwrap();
                            let mint_address = Pubkey::from_str(mint_str).unwrap();
                            let bonding_curve_address =
                                Pubkey::from_str(bonding_curve_str).unwrap();

                            Some(PumpFunInstruction::Sell((
                                args,
                                (mint_address, bonding_curve_address),
                            )))
                        }
                    },
                    None => {
                        println!(
                            "{} Unknown instruction discriminator: {:?} out of known {:?}",
                            log_tag, discriminator, known_discriminators
                        );
                        return None;
                    }
                };

                parsed_instruction
            } else {
                None
            }
        })
        .collect()
}
