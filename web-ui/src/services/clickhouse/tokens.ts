import { z } from 'zod';
import { client } from './client';

export const CLICKHOUSE_PUMP_FUN_TRADES_TABLE_NAME = 'pump_fun_trades';
export const CLICKHOUSE_TOKENS_TABLE_NAME = 'tokens';
export const CLICKHOUSE_CRAWL_STATUS_TABLE_NAME = 'crawl_status';
export const CLICKHOUSE_CURRENT_PRICES_TABLE_NAME =
  'pump_fun_trades_current_prices';

const tokenSchema = z
  .object({
    mint_address: z.string(),
    trades_count: z.number(),
    price: z.number(),
    is_complete: z.coerce.boolean(),
  })
  .transform(
    ({ mint_address: mintAddress, trades_count, is_complete, ...rest }) => {
      return {
        ...rest,
        mintAddress,
        tradesCount: trades_count,
        isComplete: is_complete,
      };
    },
  );

export type Token = z.infer<typeof tokenSchema>;

export const getTokenList = async () => {
  const result = await client.query({
    query: `SELECT 
        tokens.mint_address as mint_address,
        COALESCE(trades.trades_count, 0) as trades_count,
        any(p.price) as price,
        any(c.is_first_account_signature) as is_complete
      FROM ${CLICKHOUSE_TOKENS_TABLE_NAME} tokens
      LEFT JOIN (
        SELECT 
          mint_address,
          toUInt32(count(DISTINCT tuple(transaction_signature, instruction_index))) as trades_count
        FROM ${CLICKHOUSE_PUMP_FUN_TRADES_TABLE_NAME}
        GROUP BY mint_address
      ) trades ON tokens.mint_address = trades.mint_address
      LEFT JOIN ${CLICKHOUSE_CURRENT_PRICES_TABLE_NAME} p ON 
        tokens.mint_address = p.mint_address
      LEFT JOIN ${CLICKHOUSE_CRAWL_STATUS_TABLE_NAME} c ON 
        tokens.mint_address = c.account_address
      GROUP BY tokens.mint_address, trades.trades_count
      ORDER BY trades.trades_count DESC`,
  });

  const json = await result.json();

  if (!json.data) {
    throw new Error('No token list returned from ClickHouse');
  }

  return z.array(tokenSchema).parse(json.data);
};
