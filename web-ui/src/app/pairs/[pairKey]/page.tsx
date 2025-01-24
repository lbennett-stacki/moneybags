import Link from 'next/link';
import { getCandlesDatapoints } from '@/components/Chart/server/getCandlesDatapoints';
import {
  CandleResolution,
  DEFAULT_RESOLUTION,
  getTradeCandlesByPair,
} from '@/services/clickhouse/trades/candles';
import { ChartTypes } from '@/components/Chart/types';
import { ResolutionSelector } from '@/components/Chart/ResolutionSelector';
import { LiveChart } from '@/components/Chart/LiveChart';

const parsePairKey = (pairKey: string) => {
  const [coinTokenAddress, priceCoinTokenAddress] = pairKey.split('-');
  return {
    coinTokenAddress,
    priceCoinTokenAddress,
  };
};

export default async function PairPage({
  params,
  searchParams,
}: {
  params: Promise<{ pairKey: string }>;
  searchParams: Promise<{
    'resolution.value'?: CandleResolution['value'];
    'resolution.unit'?: CandleResolution['unit'];
  }>;
}) {
  const {
    'resolution.value': resolutionValue,
    'resolution.unit': resolutionUnit,
  } = await searchParams;
  const { pairKey } = await params;
  const pair = parsePairKey(pairKey);

  const resolution = {
    value: resolutionValue ?? DEFAULT_RESOLUTION.value,
    unit: resolutionUnit ?? DEFAULT_RESOLUTION.unit,
  };

  const candles = await getTradeCandlesByPair(pair, resolution);

  const candlesDatapoints = getCandlesDatapoints([candles], resolution);

  const nextParams = new URLSearchParams();
  nextParams.set('resolution.value', resolution.value.toString());
  nextParams.set('resolution.unit', resolution.unit);

  return (
    <>
      <div className="flex flex-col gap-4 absolute top-0 left-0 p-4 z-[200]">
        <div className="flex flex-row gap-4">
          <h1>Pair Key: {pairKey}</h1>
          <Link href={`/pairs?${nextParams}`}>Back to pairs list</Link>
        </div>
        <ResolutionSelector value={resolution} />
      </div>

      <LiveChart
        className="z-[100]"
        type={ChartTypes.Candles}
        data={candlesDatapoints}
        updateFn={async () => {
          'use server';
          const nextCandles = await getTradeCandlesByPair(pair, resolution);
          const nextDatapoints = getCandlesDatapoints(
            [nextCandles],
            resolution,
          );
          return nextDatapoints;
        }}
      />
    </>
  );
}
