import { z } from 'zod';
import { client } from '../client';
import { Pair, TRADES_TABLE_NAME } from '../pairs';

const tradeSchema = z
  .object({
    coin_token_address: z.string(),
    transaction_signature: z.string(),
    slot: z.string(),
    block_time: z.number(),
    instruction_index: z.string(),
    direction: z.string(),
    coin_token_amount: z.string(),
    price_coin_token_amount: z.string(),
  })
  .transform(
    ({
      coin_token_address: coinTokenAddress,
      transaction_signature: transactionSignature,
      slot: slot,
      block_time: blockTime,
      instruction_index: instructionIndex,
      coin_token_amount: coinTokenAmount,
      price_coin_token_amount: priceCoinTokenAmount,
      ...values
    }) => {
      return {
        ...values,
        coinTokenAddress,
        transactionSignature,
        slot: BigInt(slot),
        blockTime: new Date(blockTime * 1000),
        instructionIndex: Number(instructionIndex),
        coinTokenAmount: BigInt(coinTokenAmount),
        priceCoinTokenAmount: BigInt(priceCoinTokenAmount),
      };
    },
  );

export type Trade = z.infer<typeof tradeSchema>;

export const getTradesByPair = async (
  pair: Pick<Pair, 'coinTokenAddress' | 'priceCoinTokenAddress'>,
): Promise<Trade[]> => {
  const result = await client.query({
    query: `SELECT
      concat(coin_token_address, '-', price_coin_token_address) as pair_key,
      concat(transaction_signature, '-', instruction_index) as trade_key,
      coin_token_address,
      transaction_signature,
      slot,
      toUnixTimestamp(block_time) as block_time,
      instruction_index,
      coin_token_amount,
      price_coin_token_amount,
      direction
    FROM ${TRADES_TABLE_NAME}
    WHERE coin_token_address = '${pair.coinTokenAddress}'
      AND price_coin_token_address = '${pair.priceCoinTokenAddress}'
    ORDER BY block_time ASC`,
  });

  const json = await result.json();

  return z.array(tradeSchema).parse(json.data);
};
