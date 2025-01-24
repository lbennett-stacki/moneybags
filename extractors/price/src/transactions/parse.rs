use crate::{
    cpi::cpi::CpiLog,
    instructions::{
        instruction::InstructionDiscriminator,
        parse::{parse_single_instruction, InstructionWithLogs, ParsableInstruction},
    },
    pump_fun::cpi::PumpFunCpiLog,
    raydium::cpi::RaydiumCpiLog,
};
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::{
    option_serializer::OptionSerializer, UiInstruction, UiRawMessage, UiTransactionStatusMeta,
};
use std::collections::HashMap;

pub fn parse_transaction_with_logs(
    meta: &Option<UiTransactionStatusMeta>,
    known_discriminators: &HashMap<[u8; 8], InstructionDiscriminator>,
    account_keys: &[String],
    raw_message: &UiRawMessage,
    pump_fun_program_id: &Pubkey,
) -> Vec<InstructionWithLogs> {
    let mut result = Vec::new();

    let all_instructions = flatten_instructions(raw_message, meta);

    if let Some(meta) = meta {
        if let OptionSerializer::Some(logs) = &meta.log_messages {
            let mut current_instruction_index = 0;
            let mut current_cpi_logs = Vec::new();

            let mut stack_depth = 0;

            for log in logs {
                println!("unknown log: {:#?}", log);
                if log.contains("invoke") {
                    println!(
                        "INVOKE sd: {} ix: {}",
                        stack_depth, current_instruction_index
                    );
                    stack_depth += 1;
                } else if log.contains("success") || log.contains("failed") {
                    println!(
                        "SUCCESS/FAILED sd: {} ix: {}",
                        stack_depth, current_instruction_index
                    );
                    stack_depth -= 1;

                    if stack_depth == 0 {
                        let instruction = if current_instruction_index < all_instructions.len() {
                            let ix = &all_instructions[current_instruction_index];
                            println!(
                                "current instruction: {:#?} at index {}",
                                ix, current_instruction_index
                            );
                            parse_single_instruction(
                                known_discriminators,
                                account_keys,
                                ix.program_id_index,
                                &ix.data,
                                &ix.accounts,
                                current_instruction_index as u64,
                                pump_fun_program_id,
                            )
                        } else {
                            None
                        };

                        println!("parsed instruction: {:#?}", instruction);

                        result.push(InstructionWithLogs {
                            instruction,
                            cpi_logs: current_cpi_logs.clone(),
                        });

                        current_cpi_logs.clear();
                        current_instruction_index += 1;
                    }
                } else if let Some(cpi_log) = PumpFunCpiLog::from_encoded_log(log) {
                    println!("pumpfun cpi log: {:#?}", cpi_log);
                    current_cpi_logs.push(CpiLog::PumpFun(cpi_log));
                } else if let Some(cpi_log) = RaydiumCpiLog::from_encoded_log(log) {
                    println!("raydium cpi log: {:#?}", cpi_log);
                    current_cpi_logs.push(CpiLog::Raydium(cpi_log));
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
                    inner_ix_set.index,
                    inner_ix_set
                        .instructions
                        .iter()
                        .filter_map(|ix| match ix {
                            UiInstruction::Compiled(ix) => Some(ParsableInstruction {
                                data: ix.data.clone(),
                                accounts: ix.accounts.clone(),
                                program_id_index: ix.program_id_index as usize,
                            }),
                            _ => None,
                        })
                        .collect::<Vec<_>>(),
                );
            }
        }
    }

    println!(
        "inner_instructions_by_instruction_index: {:#?}",
        inner_instructions_by_instruction_index
    );

    raw_message
        .instructions
        .iter()
        .flat_map(|ix| {
            let main = ParsableInstruction {
                data: ix.data.clone(),
                accounts: ix.accounts.clone(),
                program_id_index: ix.program_id_index as usize,
            };

            let empty_vec = Vec::new();
            let inner_instructions = inner_instructions_by_instruction_index
                .get(&ix.program_id_index)
                .unwrap_or(&empty_vec);

            let mut all = vec![main];
            all.extend(inner_instructions.iter().map(|iix| ParsableInstruction {
                data: iix.data.clone(),
                accounts: iix.accounts.clone(),
                program_id_index: iix.program_id_index as usize,
            }));

            all
        })
        .collect::<Vec<_>>()
}
