use crate::{
    cpi::cpi::CpiLog,
    instructions::parse::{parse_single_instruction, InstructionWithLogs, ParsableInstruction},
    pump_fun::cpi::PumpFunCpiLog,
};
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::{
    option_serializer::OptionSerializer, UiInstruction, UiRawMessage, UiTransactionStatusMeta,
};
use std::collections::HashMap;

pub fn parse_transaction_with_logs(
    meta: &Option<UiTransactionStatusMeta>,
    raw_message: &UiRawMessage,
    pump_fun_program_id: &Pubkey,
    raydium_amm_program_id: &Pubkey,
) -> Vec<InstructionWithLogs> {
    let mut result = Vec::new();

    let account_keys = &raw_message.account_keys;
    let all_instructions = flatten_instructions(raw_message, meta);

    if let Some(meta) = meta {
        if let OptionSerializer::Some(logs) = &meta.log_messages {
            let mut current_instruction_index = 0;
            let mut current_cpi_logs = Vec::new();

            let mut stack_depth = 0;

            for log in logs {
                if log.contains("invoke") {
                    stack_depth += 1;
                } else if log.contains("success") || log.contains("failed") {
                    stack_depth -= 1;

                    if stack_depth == 0 {
                        let parsable_ix = &all_instructions[current_instruction_index];
                        let instruction = if current_instruction_index < all_instructions.len() {
                            parse_single_instruction(
                                &parsable_ix,
                                account_keys,
                                current_instruction_index as u64,
                                pump_fun_program_id,
                                raydium_amm_program_id,
                                &all_instructions,
                            )
                        } else {
                            None
                        };

                        result.push(InstructionWithLogs {
                            instruction,
                            cpi_logs: current_cpi_logs.clone(),
                        });

                        if let Some(inner_instructions) = &parsable_ix.inner_instructions {
                            for inner_ix in inner_instructions {
                                let instruction = parse_single_instruction(
                                    &inner_ix,
                                    account_keys,
                                    inner_ix.instruction_index as u64,
                                    pump_fun_program_id,
                                    raydium_amm_program_id,
                                    &inner_instructions,
                                );

                                result.push(InstructionWithLogs {
                                    instruction,
                                    cpi_logs: current_cpi_logs.clone(),
                                });
                            }
                        }

                        current_cpi_logs.clear();
                        current_instruction_index += 1;
                    }
                } else if let Some(cpi_log) = PumpFunCpiLog::from_encoded_log(log) {
                    current_cpi_logs.push(CpiLog::PumpFun(cpi_log));
                }
            }
        }
    }

    result
}

fn flatten_instructions(
    raw_message: &UiRawMessage,
    meta: &Option<UiTransactionStatusMeta>,
) -> Vec<ParsableInstruction> {
    let mut inner_instructions_by_instruction_index = HashMap::new();

    if let Some(meta) = meta {
        if let OptionSerializer::Some(inner_instructions) = &meta.inner_instructions {
            for inner_ix_set in inner_instructions {
                inner_instructions_by_instruction_index.insert(
                    inner_ix_set.index as usize,
                    inner_ix_set
                        .instructions
                        .iter()
                        .enumerate()
                        .filter_map(|(inner_instruction_index, ix)| match ix {
                            UiInstruction::Compiled(ix) => Some(ParsableInstruction {
                                data: ix.data.clone(),
                                accounts: ix.accounts.clone(),
                                program_id_index: ix.program_id_index as usize,
                                inner_instructions: None,
                                instruction_index: inner_ix_set.index as usize,
                                inner_instruction_index: Some(inner_instruction_index),
                            }),
                            _ => None,
                        })
                        .collect::<Vec<_>>(),
                );
            }
        }
    }

    raw_message
        .instructions
        .iter()
        .enumerate()
        .map(|(instruction_index, ix)| {
            let empty_vec = Vec::new();
            let inner_instructions = inner_instructions_by_instruction_index
                .get(&instruction_index)
                .unwrap_or(&empty_vec);

            let new_inner_instructions = inner_instructions
                .clone()
                .iter()
                .enumerate()
                .map(|(inner_instruction_index, iix)| ParsableInstruction {
                    data: iix.data.clone(),
                    accounts: iix.accounts.clone(),
                    program_id_index: iix.program_id_index as usize,
                    inner_instructions: None,
                    instruction_index,
                    inner_instruction_index: Some(inner_instruction_index),
                })
                .collect::<Vec<_>>();

            let parsable = ParsableInstruction {
                data: ix.data.clone(),
                accounts: ix.accounts.clone(),
                program_id_index: ix.program_id_index as usize,
                inner_instructions: Some(new_inner_instructions.clone()),
                instruction_index,
                inner_instruction_index: None,
            };

            parsable
        })
        .collect::<Vec<_>>()
}
