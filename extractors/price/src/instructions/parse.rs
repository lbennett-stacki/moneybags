use crate::{
    cpi::cpi::CpiLog,
    instructions::instruction::Instruction,
    pump_fun::{
        discriminators::PumpFunInstructionDiscriminator,
        idl::{
            BuyAccountsOrder, BuyInstructionArgs, CreateAccountsOrder, CreateInstructionArgs,
            SellAccountsOrder, SellInstructionArgs,
        },
        instructions::PumpFunInstruction,
    },
    raydium::{
        discriminators::RaydiumInstructionDiscriminator,
        idl::{
            SwapBaseInAccountsOrder, SwapBaseInInstructionArgs, SwapBaseOutAccountsOrder,
            SwapBaseOutInstructionArgs,
        },
        instructions::RaydiumInstruction,
    },
};
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;
use std::{collections::HashMap, str::FromStr};

use super::instruction::InstructionDiscriminator;

#[derive(Debug)]
pub struct InstructionWithLogs {
    pub instruction: Option<Instruction>,
    pub cpi_logs: Vec<CpiLog>,
}

#[derive(Debug)]
pub struct ParsableInstruction {
    pub data: String,
    pub accounts: Vec<u8>,
    pub program_id_index: usize,
}

pub fn parse_single_instruction(
    known_discriminators: &HashMap<[u8; 8], InstructionDiscriminator>,
    account_keys: &[String],
    executing_program_id_index: usize,
    data: &str,
    instruction_accounts: &Vec<u8>,
    instruction_index: u64,
    pump_fun_program_id: &Pubkey,
) -> Option<Instruction> {
    if let Some(executing_program_id_str) = account_keys.get(executing_program_id_index) {
        // if executing_program_id_str != &pump_fun_program_id.to_string() {
        //     println!(
        //         "executing program id mismatch (not pump fun prog) {:#?}",
        //         executing_program_id_str
        //     );
        //     // executing program id mismatch (not pump fun prog)
        //     return None;
        // }

        println!("data to decode: {:#?}", data);
        let data = bs58::decode(&data).into_vec().ok()?;
        if data.len() < 8 {
            println!("data length mismatch (no anchor discriminator?)");
            // data length mismatch (not pump fun inst?)
            return None;
        }

        let discriminator: [u8; 8] = data[..8].try_into().unwrap();
        let found_discriminator = known_discriminators.get(&discriminator);
        println!("discriminator found: {:#?}", found_discriminator);

        match found_discriminator {
            Some(instruction) => match instruction {
                InstructionDiscriminator::PumpFun(PumpFunInstructionDiscriminator::Create) => {
                    let args = CreateInstructionArgs::try_from_slice(&data[8..]).unwrap();

                    // TODO: We now have found the name, symbol and metadata uri of a coin,
                    // we should insert into db
                    //and trigger a scrape of the metadata uri for json payload returning name, symbol, description, image uri, showName, createdOn, twitter, and maybe more?

                    let mint = instruction_accounts
                        .get(CreateAccountsOrder::Mint as usize)
                        .unwrap();
                    let bonding_curve = instruction_accounts
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

                // TODO: these buy and sell branches are a valued transcation in themselves, so we should insert into a tx table to avoid re-reading/parsing etc
                InstructionDiscriminator::PumpFun(PumpFunInstructionDiscriminator::Buy) => {
                    let args = BuyInstructionArgs::try_from_slice(&data[8..]).unwrap();

                    let mint = instruction_accounts
                        .get(BuyAccountsOrder::Mint as usize)
                        .unwrap();
                    let bonding_curve = instruction_accounts
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
                InstructionDiscriminator::PumpFun(PumpFunInstructionDiscriminator::Sell) => {
                    let args = SellInstructionArgs::try_from_slice(&data[8..]).unwrap();

                    let mint = instruction_accounts
                        .get(SellAccountsOrder::Mint as usize)
                        .unwrap();
                    let bonding_curve = instruction_accounts
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
                InstructionDiscriminator::Raydium(RaydiumInstructionDiscriminator::SwapBaseIn) => {
                    println!("Raydium SWAP BASE IN instru");
                    let args = SwapBaseInInstructionArgs::try_from_slice(&data[8..]).unwrap();

                    let coin_token = instruction_accounts
                        .get(SwapBaseInAccountsOrder::PoolCoinTokenAccount as usize)
                        .unwrap();
                    let coin_token_str = account_keys.get(*coin_token as usize).unwrap();
                    let coin_token_address = Pubkey::from_str(coin_token_str).unwrap();

                    let pc_token = instruction_accounts
                        .get(SwapBaseInAccountsOrder::PoolPcTokenAccount as usize)
                        .unwrap();
                    let pc_token_str = account_keys.get(*pc_token as usize).unwrap();
                    let pc_token_address = Pubkey::from_str(pc_token_str).unwrap();

                    Some(Instruction::Raydium(
                        instruction_index,
                        RaydiumInstruction::SwapBaseIn((
                            args,
                            (coin_token_address, pc_token_address),
                        )),
                    ))
                }
                InstructionDiscriminator::Raydium(RaydiumInstructionDiscriminator::SwapBaseOut) => {
                    println!("Raydium SWAP BASE OUT instru");
                    let args = SwapBaseOutInstructionArgs::try_from_slice(&data[8..]).unwrap();

                    let coin_token = instruction_accounts
                        .get(SwapBaseOutAccountsOrder::PoolCoinTokenAccount as usize)
                        .unwrap();
                    let coin_token_str = account_keys.get(*coin_token as usize).unwrap();
                    let coin_token_address = Pubkey::from_str(coin_token_str).unwrap();

                    let pc_token = instruction_accounts
                        .get(SwapBaseOutAccountsOrder::PoolPcTokenAccount as usize)
                        .unwrap();
                    let pc_token_str = account_keys.get(*pc_token as usize).unwrap();
                    let pc_token_address = Pubkey::from_str(pc_token_str).unwrap();

                    Some(Instruction::Raydium(
                        instruction_index,
                        RaydiumInstruction::SwapBaseOut((
                            args,
                            (coin_token_address, pc_token_address),
                        )),
                    ))
                }
            },
            None => None,
        }
    } else {
        // no executing program id found
        None
    }
}
