use super::{
    discriminators::RaydiumInstructionDiscriminator,
    idl::{SwapBaseInInstructionArgs, SwapBaseOutInstructionArgs},
};
use crate::{
    instructions::{instruction::Instruction, parse::ParsableInstruction},
    raydium::{
        discriminators::build_raydium_instruction_discriminators,
        idl::{SwapBaseInAccountsOrder, SwapBaseOutAccountsOrder},
    },
    system::idl::{SystemTransferAccountsOrder, SystemTransferInstructionData},
    token::idl::{TokenTransferAccountsOrder, TokenTransferInstructionData},
    utils::log::log_time,
};
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub type PoolCoinTokenAddress = Pubkey;
pub type PoolPcTokenAddress = Pubkey;
pub type RaydiumToken = (PoolCoinTokenAddress, PoolPcTokenAddress);
pub type RaydiumTokenAmounts = (u64, u64);

#[derive(Debug)]
pub enum RaydiumInstruction {
    SwapBaseIn((SwapBaseInInstructionArgs, RaydiumToken, RaydiumTokenAmounts)),
    SwapBaseOut(
        (
            SwapBaseOutInstructionArgs,
            RaydiumToken,
            RaydiumTokenAmounts,
        ),
    ),
}

pub fn parse_single_raydium_instruction(
    parsable: &ParsableInstruction,
    account_keys: &[String],
    instruction_index: u64,
    inner_instructions: &Vec<ParsableInstruction>,
) -> Option<Instruction> {
    let data = bs58::decode(&parsable.data).into_vec().ok()?;
    if data.len() < 1 {
        return None;
    }

    let discriminator = data[0];
    let raydium_discriminators = build_raydium_instruction_discriminators();

    let found_discriminator = raydium_discriminators.get(&discriminator);

    let instruction = match found_discriminator {
        None => None,
        Some(RaydiumInstructionDiscriminator::SwapBaseIn) => {
            let args = SwapBaseInInstructionArgs::deserialize(&mut &data[1..]).unwrap();

            let inner_instructions = if let Some(inner_instructions) = &parsable.inner_instructions
            {
                inner_instructions.clone()
            } else {
                let next_inst =
                    inner_instructions.get(parsable.inner_instruction_index.unwrap() as usize + 1);
                let next_next_inst =
                    inner_instructions.get(parsable.inner_instruction_index.unwrap() as usize + 2);

                if next_inst.is_none() || next_next_inst.is_none() {
                    vec![]
                } else {
                    vec![next_inst.unwrap().clone(), next_next_inst.unwrap().clone()]
                }
            };

            let transfers = inner_instructions
                .iter()
                .filter_map(|ix| parse_inner_transfer_instruction(ix, account_keys))
                .collect::<Vec<_>>();

            let coin_token = parsable
                .accounts
                .get(SwapBaseInAccountsOrder::PoolCoinTokenAccount as usize)?;
            let coin_token_str = account_keys.get(*coin_token as usize)?;
            let coin_token_address = Pubkey::from_str(coin_token_str).ok()?;

            let pc_token = parsable
                .accounts
                .get(SwapBaseInAccountsOrder::PoolPcTokenAccount as usize)?;
            let pc_token_str = account_keys.get(*pc_token as usize)?;
            let pc_token_address = Pubkey::from_str(pc_token_str).ok()?;

            let (coin_token_amount, pc_token_amount) =
                token_amounts_from_transfers(transfers, coin_token_str, pc_token_str)?;

            Some(Instruction::Raydium(
                instruction_index,
                RaydiumInstruction::SwapBaseIn((
                    args,
                    (coin_token_address, pc_token_address),
                    (coin_token_amount, pc_token_amount),
                )),
            ))
        }
        Some(RaydiumInstructionDiscriminator::SwapBaseOut) => {
            let args = SwapBaseOutInstructionArgs::deserialize(&mut &data[1..]).unwrap();

            let inner_instructions = parsable.inner_instructions.clone().unwrap();
            let transfers = inner_instructions
                .iter()
                .filter_map(|ix| parse_inner_transfer_instruction(ix, account_keys))
                .collect::<Vec<_>>();

            let coin_token = parsable
                .accounts
                .get(SwapBaseOutAccountsOrder::PoolCoinTokenAccount as usize)
                .unwrap();
            let coin_token_str = account_keys.get(*coin_token as usize).unwrap();
            let coin_token_address = Pubkey::from_str(coin_token_str).unwrap();

            let pc_token = parsable
                .accounts
                .get(SwapBaseOutAccountsOrder::PoolPcTokenAccount as usize)
                .unwrap();
            let pc_token_str = account_keys.get(*pc_token as usize).unwrap();
            let pc_token_address = Pubkey::from_str(pc_token_str).unwrap();

            let (coin_token_amount, pc_token_amount) =
                token_amounts_from_transfers(transfers, coin_token_str, pc_token_str)?;

            Some(Instruction::Raydium(
                instruction_index,
                RaydiumInstruction::SwapBaseOut((
                    args,
                    (coin_token_address, pc_token_address),
                    (coin_token_amount, pc_token_amount),
                )),
            ))
        }
    };

    return instruction;
}

pub type TransferSource = String;
pub type TransferDestination = String;
pub type TransferAmount = u64;

fn parse_inner_transfer_instruction(
    ix: &ParsableInstruction,
    account_keys: &[String],
) -> Option<(TransferAmount, TransferSource, TransferDestination)> {
    let ix_data = bs58::decode(&ix.data).into_vec().ok()?;

    if ix.program_id_index == 0 {
        if ix_data[0] != 2 {
            None
        } else {
            let amount = SystemTransferInstructionData::deserialize(&mut &ix_data[1..]).ok()?;

            let source_index = ix
                .accounts
                .get(SystemTransferAccountsOrder::Source as usize)?;

            let destination_index = ix
                .accounts
                .get(SystemTransferAccountsOrder::Destination as usize)?;

            let source = account_keys.get(*source_index as usize)?;
            let destination = account_keys.get(*destination_index as usize)?;

            Some((amount.lamports, source.to_string(), destination.to_string()))
        }
    } else {
        if ix_data[0] != 3 {
            None
        } else {
            let amount = TokenTransferInstructionData::deserialize(&mut &ix_data[1..]).ok()?;

            let source_index = ix
                .accounts
                .get(TokenTransferAccountsOrder::Source as usize)?;

            let destination_index = ix
                .accounts
                .get(TokenTransferAccountsOrder::Destination as usize)?;

            let source = account_keys.get(*source_index as usize)?;
            let destination = account_keys.get(*destination_index as usize)?;

            Some((amount.amount, source.to_string(), destination.to_string()))
        }
    }
}

fn token_amounts_from_transfers(
    transfers: Vec<(u64, String, String)>,
    coin_token_str: &str,
    pc_token_str: &str,
) -> Option<(u64, u64)> {
    if transfers.len() != 2 {
        return None;
    }

    let (first_amount, _, first_dest) = transfers.first().unwrap();
    let (second_amount, second_src, _) = transfers.last().unwrap();

    match (first_dest.as_str(), second_src.as_str()) {
        (dest, src) if dest == coin_token_str && src == pc_token_str => {
            Some((*first_amount, *second_amount))
        }
        (dest, src) if dest == pc_token_str && src == coin_token_str => {
            Some((*second_amount, *first_amount))
        }
        _ => {
            println!(
                "{} Transfer addresses don't match expected token pairs",
                log_time()
            );
            None
        }
    }
}
