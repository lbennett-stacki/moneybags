import { getCumulativeDatapoints } from '@/components/Chart/server/getCumulativeDatapoints';
import { SyncedCharts } from '@/components/Chart/SyncedCharts';
import { LivePairsList } from '@/components/PairsList/LivePairsList';
import { getPairsList } from '@/services/clickhouse/pairs';
import {
  CandleResolution,
  DEFAULT_RESOLUTION,
} from '@/services/clickhouse/trades/candles';
import {
  getCumulativePairsCount,
  getCumulativeTradeCount,
} from '@/services/clickhouse/trades/cumulative';
import { Time } from 'lightweight-charts';

export default async function PairsPage({
  searchParams,
}: {
  searchParams: Promise<{
    'resolution.value'?: CandleResolution['value'];
    'resolution.unit'?: CandleResolution['unit'];
  }>;
}) {
  const {
    'resolution.value': resolutionValue,
    'resolution.unit': resolutionUnit,
  } = await searchParams;

  const resolution = {
    value: resolutionValue ?? DEFAULT_RESOLUTION.value,
    unit: resolutionUnit ?? DEFAULT_RESOLUTION.unit,
  };

  const pairs = await getPairsList();
  const pairsCount = await getCumulativePairsCount();
  const tradesCount = await getCumulativeTradeCount();

  const pairsData = getCumulativeDatapoints(
    [
      pairsCount.map((pair) => ({
        time: (pair.time_bucket.getTime() / 1000) as Time,
        value: pair.count,
      })),
    ],
    resolution,
  );
  const tradesData = getCumulativeDatapoints(
    [
      tradesCount.map((trade) => ({
        time: (trade.time_bucket.getTime() / 1000) as Time,
        value: trade.count,
      })),
    ],
    resolution,
  );

  return (
    <>
      <h1>Pairs Page</h1>

      <SyncedCharts
        charts={[
          {
            height: 150,
            data: [pairsData[0]!],
            onUpdate: async () => {
              'use server';
              const nextPairsCount = await getCumulativePairsCount();
              const nextPairsData = getCumulativeDatapoints(
                [
                  nextPairsCount.map((pair) => ({
                    time: (pair.time_bucket.getTime() / 1000) as Time,
                    value: pair.count,
                  })),
                ],
                resolution,
              );
              return [nextPairsData[0]!];
            },
          },
          {
            height: 300,
            data: [tradesData[0]!],
            onUpdate: async () => {
              'use server';
              const nextTradesCount = await getCumulativeTradeCount();
              const nextTradesData = getCumulativeDatapoints(
                [
                  nextTradesCount.map((trade) => ({
                    time: (trade.time_bucket.getTime() / 1000) as Time,
                    value: trade.count,
                  })),
                ],
                resolution,
              );
              return [nextTradesData[0]!];
            },
          },
        ]}
      />

      <LivePairsList
        pairs={pairs}
        updateFn={async () => {
          'use server';
          const nextPairs = await getPairsList();
          return nextPairs;
        }}
      />
    </>
  );
}
