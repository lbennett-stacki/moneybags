import { z } from 'zod';
import { client } from '../client';
import { TRADES_TABLE_NAME } from '../pairs';

const cumulativeCountSchema = z.object({
  time_bucket: z.coerce.date(),
  count: z.coerce.number(),
});

export type CumulativeCount = z.infer<typeof cumulativeCountSchema>;

export const getCumulativePairsCount = async (): Promise<CumulativeCount[]> => {
  const result = await client.query({
    query: `
      SELECT 
        toStartOfMinute(block_time) as time_bucket,
        count(DISTINCT (coin_token_address, price_coin_token_address)) OVER (ORDER BY toStartOfMinute(block_time)) as count
      FROM ${TRADES_TABLE_NAME}
      GROUP BY time_bucket, coin_token_address, price_coin_token_address
      ORDER BY time_bucket ASC
    `,
  });

  const json = await result.json();
  return z.array(cumulativeCountSchema).parse(json.data);
};

export const getCumulativeTradeCount = async (): Promise<CumulativeCount[]> => {
  const result = await client.query({
    query: `
      SELECT 
        toStartOfMinute(block_time) as time_bucket,
        count(*) OVER (ORDER BY toStartOfMinute(block_time)) as count
      FROM ${TRADES_TABLE_NAME}
      GROUP BY time_bucket
      ORDER BY time_bucket ASC
    `,
  });

  const json = await result.json();
  return z.array(cumulativeCountSchema).parse(json.data);
};
