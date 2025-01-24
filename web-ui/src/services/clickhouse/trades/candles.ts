import { z } from 'zod';
import { client } from '../client';
import { TRADES_TABLE_NAME, Pair } from '../pairs';

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
    coin_token_address: z.string(),
    price_coin_token_address: z.string(),
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
      coin_token_address: coinTokenAddress,
      price_coin_token_address: priceCoinTokenAddress,
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
        coinTokenAddress,
        priceCoinTokenAddress,
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
  pair: Pick<Pair, 'coinTokenAddress' | 'priceCoinTokenAddress'>,
): Promise<Candle[]> => {
  const whereClause = `WHERE coin_token_address = '${pair.coinTokenAddress}' AND price_coin_token_address = '${pair.priceCoinTokenAddress}'`;

  const result = await client.query({
    query: `
SELECT
  tra.coin_token_address as coin_token_address,
  tra.price_coin_token_address as price_coin_token_address,
  toStartOfInterval(toDateTime64(tra.block_time, 0), INTERVAL ${resolution.value} ${resolution.unit.toUpperCase()}) as time_bucket,
  coalesce(min(tra.price_coin_token_amount/nullIf(tra.coin_token_amount, 0)), 0) as low_price,
  coalesce(max(tra.price_coin_token_amount/nullIf(tra.coin_token_amount, 0)), 0) as high_price,
  coalesce(argMin(tra.price_coin_token_amount/nullIf(tra.coin_token_amount, 0), (tra.block_time, tra.slot, tra.instruction_index)), 0) as open_price,
  coalesce(argMax(tra.price_coin_token_amount/nullIf(tra.coin_token_amount, 0), (tra.block_time, tra.slot, tra.instruction_index)), 0) as close_price,
  sum(if(tra.direction = 'buy', tra.coin_token_amount, 0)) as buy_volume,
  sum(if(tra.direction = 'sell', tra.coin_token_amount, 0)) as sell_volume,
  sum(tra.coin_token_amount) as volume,
  count() as trade_count
FROM (
   SELECT DISTINCT
       coin_token_address, price_coin_token_address, block_time, slot, instruction_index, direction, 
       price_coin_token_amount, coin_token_amount, transaction_signature
   FROM ${TRADES_TABLE_NAME} as tra
   ${whereClause}
) as tra
GROUP BY tra.coin_token_address, tra.price_coin_token_address, time_bucket
ORDER BY time_bucket ASC;
    `,
  });

  const json = await result.json();

  return z.array(candleSchema).parse(json.data);
};

export const getTradeCandlesByPair = async (
  pair: Pick<Pair, 'coinTokenAddress' | 'priceCoinTokenAddress'>,
  resolution: CandleResolution,
): Promise<Candle[]> => {
  return getCandles(resolution, pair);
};
