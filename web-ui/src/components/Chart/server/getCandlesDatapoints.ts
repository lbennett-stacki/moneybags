import { Candle } from '@/services/clickhouse/trades/candles';
import { LineData, Time } from 'lightweight-charts';

const LAMPORTS_PER_SOL = 1_000_000_000;

export const getCandlesDatapoints = (candlesBuckets: Candle[][]) => {
  return candlesBuckets.map((candles) => {
    const v: LineData[] = [];
    const c = candles.map((candle, i, all) => {
      const prev = all[i];

      const timeBucketModifier =
        prev && prev.timeBucket.getTime() === candle.timeBucket.getTime()
          ? i + 1
          : 0;

      const time = (candle.timeBucket.getTime() / 1000 +
        timeBucketModifier) as Time;

      v.push({
        time,
        value: Number(candle.volume / BigInt(LAMPORTS_PER_SOL)),
      });

      return {
        time,
        open: candle.openPrice,
        high: candle.highPrice,
        low: candle.lowPrice,
        close: candle.closePrice,
      };
    });

    return { candles: c, volumes: v };
  });
};
