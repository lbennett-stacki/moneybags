import { z } from 'zod';
import { client } from '../client';
import {
  CLICKHOUSE_PUMP_FUN_TRADES_TABLE_NAME,
  CLICKHOUSE_TOKENS_TABLE_NAME,
} from '../tokens';

export type ClickhouseInterval = 'second' | 'minute' | 'hour' | 'day';

export const candleResolutionUnits: ClickhouseInterval[] = [
  'second',
  'minute',
  'hour',
  'day',
];

export const DEFAULT_RESOLUTION: CandleResolution = {
  value: 5,
  unit: 'minute',
};

export const isCandleResolutionUnit = (
  value: string,
): value is CandleResolution['unit'] => {
  return candleResolutionUnits.includes(value as ClickhouseInterval);
};

export const isCandleResolutionValue = (
  value: string | number,
): value is CandleResolution['value'] => {
  const numberValue = typeof value === 'number' ? value : Number(value);

  return numberValue > 0 && Number.isInteger(numberValue);
};

export interface CandleResolution {
  value: number;
  unit: ClickhouseInterval;
}

export const candleSchema = z
  .object({
    mint_address: z.string(),
    time_bucket: z.coerce.date(),
    open_price: z.number(),
    close_price: z.number(),
    high_price: z.number(),
    low_price: z.number(),
    buy_volume: z.string(),
    sell_volume: z.string(),
    volume: z.string(),
  })
  .transform(
    ({
      mint_address: mintAddress,
      time_bucket: timeBucket,
      open_price: openPrice,
      close_price: closePrice,
      high_price: highPrice,
      low_price: lowPrice,
      buy_volume: buyVolume,
      sell_volume: sellVolume,
      volume,
    }) => {
      return {
        mintAddress,
        timeBucket,
        openPrice: openPrice,
        closePrice: closePrice,
        highPrice: highPrice,
        lowPrice: lowPrice,
        buyVolume: BigInt(buyVolume),
        sellVolume: BigInt(sellVolume),
        volume: BigInt(volume),
      };
    },
  );

export type Candle = z.infer<typeof candleSchema>;

const getCandles = async (
  resolution: CandleResolution,
  mintAddress?: string,
) => {
  const whereClause = mintAddress
    ? `WHERE mint_address = '${mintAddress}'`
    : '';

  const result = await client.query({
    query: `
SELECT
   tra.mint_address,
   toStartOfInterval(toDateTime64(tra.block_time, 0), INTERVAL ${resolution.value} ${resolution.unit.toUpperCase()}) as time_bucket,
   min(tra.lamports_amount/(tra.token_amount * pow(10, tok.decimals))) as low_price,
   max(tra.lamports_amount/(tra.token_amount * pow(10, tok.decimals))) as high_price,
   argMin(tra.lamports_amount/(tra.token_amount * pow(10, tok.decimals)), (tra.block_time, tra.slot, tra.instruction_index)) as open_price,
   argMax(tra.lamports_amount/(tra.token_amount * pow(10, tok.decimals)), (tra.block_time,tra.slot, tra.instruction_index)) as close_price,
   sum(if(tra.direction = 'buy', tra.token_amount, 0)) as buy_volume,
   sum(if(tra.direction = 'sell', tra.token_amount, 0)) as sell_volume,
   sum(tra.token_amount) as volume,
   count() as trade_count
FROM (
   SELECT DISTINCT
       mint_address, block_time, slot, instruction_index, direction, 
       lamports_amount, token_amount, transaction_signature
   FROM ${CLICKHOUSE_PUMP_FUN_TRADES_TABLE_NAME} as tra
   ${whereClause}
) as tra
JOIN ${CLICKHOUSE_TOKENS_TABLE_NAME} as tok ON tra.mint_address = tok.mint_address
GROUP BY tra.mint_address, time_bucket
ORDER BY time_bucket ASC;
    `,
  });

  const json = await result.json();

  return z.array(candleSchema).parse(json.data);
};

export const getPumpFunTradeCandlesByToken = async (
  mintAddress: string,
  resolution: CandleResolution,
) => {
  return getCandles(resolution, mintAddress);
};

export const getAllPumpFunTradeCandles = async (
  resolution: CandleResolution,
) => {
  return getCandles(resolution);
};
