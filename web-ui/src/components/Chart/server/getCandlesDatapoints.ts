import { Candle, CandleResolution } from '@/services/clickhouse/trades/candles';
import { LineData, Time } from 'lightweight-charts';
import { getEmptyTimesteps } from './getEmptyTimesteps';
import { CandleData } from '../props';

export const getCandlesDatapoints = (
  candlesBuckets: Candle[][],
  resolution: CandleResolution,
): CandleData[] => {
  return candlesBuckets.map((bucketCandles) => {
    const volumes: LineData[] = [];

    const candles = bucketCandles.map((candle, i, all) => {
      const prev = all[i];

      const timeBucketModifier =
        prev && prev.timeBucket.getTime() === candle.timeBucket.getTime()
          ? i + 1
          : 0;

      const time = (candle.timeBucket.getTime() / 1000 +
        timeBucketModifier) as Time;

      volumes.push({
        time,
        value: Number(candle.volume / BigInt(1_000_000_000)),
      });

      return {
        time,
        open: candle.openPrice,
        high: candle.highPrice,
        low: candle.lowPrice,
        close: candle.closePrice,
      };
    });

    const lastDatapoint = candles[candles.length - 1]!;

    const emptyDatapointCandles = getEmptyTimesteps(
      lastDatapoint,
      resolution,
    ).map((datapoint) => {
      return {
        ...datapoint,
        open: lastDatapoint.close,
        high: lastDatapoint.close,
        low: lastDatapoint.close,
        close: lastDatapoint.close,
      };
    });

    const emptyDatapointVolumes = getEmptyTimesteps(
      lastDatapoint,
      resolution,
    ).map((datapoint) => {
      return {
        time: datapoint.time,
        value: 0,
      };
    });

    const allCandles = [...candles, ...emptyDatapointCandles];
    const allVolumes = [...volumes, ...emptyDatapointVolumes];

    return {
      candles: allCandles,
      volumes: allVolumes,
    };
  });
};
