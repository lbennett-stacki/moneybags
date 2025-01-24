import { z } from 'zod';
import { client } from '../client';

const CLICKHOUSE_PUMP_FUN_TRADES_TABLE_NAME = 'pump_fun_trades';

const pumpFunTradeSchema = z
  .object({
    mint_address: z.string(),
    transaction_signature: z.string(),
    slot: z.string(),
    block_time: z.number(),
    instruction_index: z.string(),
    direction: z.string(),
    lamports_amount: z.string(),
    token_amount: z.string(),
  })
  .transform(
    ({
      mint_address: mintAddress,
      transaction_signature: transactionSignature,
      slot: slot,
      block_time: blockTime,
      instruction_index: instructionIndex,
      lamports_amount: lamportsAmount,
      token_amount: tokenAmount,
    }) => {
      return {
        mintAddress,
        transactionSignature,
        slot: BigInt(slot),
        blockTime: new Date(blockTime * 1000),
        instructionIndex: Number(instructionIndex),
        lamportsAmount: BigInt(lamportsAmount),
        tokenAmount: BigInt(tokenAmount),
      };
    },
  );

export type Trade = z.infer<typeof pumpFunTradeSchema>;

export const getPumpFunTradesByToken = async (mintAddress: string) => {
  const result = await client.query({
    query: `SELECT
      mint_address,
      transaction_signature,
      slot,
      toUnixTimestamp(block_time) as block_time,
      instruction_index,
      lamports_amount,
      token_amount,
      direction
    FROM ${CLICKHOUSE_PUMP_FUN_TRADES_TABLE_NAME}
    WHERE mint_address = '${mintAddress}'
    ORDER BY block_time ASC`,
  });

  const json = await result.json();

  return z.array(pumpFunTradeSchema).parse(json.data);
};

export const getAllPumpFunTrades = async () => {
  const result = await client.query({
    query: `SELECT
      mint_address,
      transaction_signature,
      slot,
      toUnixTimestamp(block_time) as block_time,
      instruction_index,
      lamports_amount,
      token_amount,
      direction
    FROM ${CLICKHOUSE_PUMP_FUN_TRADES_TABLE_NAME}
    ORDER BY block_time ASC`,
  });

  const json = await result.json();

  return z.array(pumpFunTradeSchema).parse(json.data);
};
