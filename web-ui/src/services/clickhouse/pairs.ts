import { z } from 'zod';
import { client } from './client';

export const TRADES_TABLE_NAME = 'trades';
export const TOKENS_TABLE_NAME = 'tokens';
export const CRAWL_STATUS_TABLE_NAME = 'crawl_status';
export const CURRENT_PRICES_TABLE_NAME = 'trades_current_prices';

const pairSchema = z
  .object({
    pair_key: z.string(),
    coin_token_address: z.string(),
    price_coin_token_address: z.string(),
    trades_count: z.number(),
  })
  .transform(
    ({
      pair_key: pairKey,
      coin_token_address: coinTokenAddress,
      price_coin_token_address: priceCoinTokenAddress,
      trades_count,
      ...rest
    }) => {
      return {
        ...rest,
        pairKey,
        coinTokenAddress,
        priceCoinTokenAddress,
        tradesCount: trades_count,
      };
    },
  );

export type Pair = z.infer<typeof pairSchema>;

export const getPairsList = async (): Promise<Pair[]> => {
  const result = await client.query({
    query: `
      SELECT 
          concat(coin_token_address, '-', price_coin_token_address) as pair_key,
          coin_token_address,
          price_coin_token_address,
          toUInt32(count(DISTINCT tuple(transaction_signature, instruction_index))) as trades_count
      FROM ${TRADES_TABLE_NAME}
      GROUP BY coin_token_address, price_coin_token_address
      ORDER BY trades_count DESC
    `,
  });

  const json = await result.json();

  if (!json.data) {
    throw new Error('No token list returned from ClickHouse');
  }

  return z.array(pairSchema).parse(json.data);
};
