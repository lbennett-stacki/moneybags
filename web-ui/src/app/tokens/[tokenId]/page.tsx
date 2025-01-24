import Link from 'next/link';
import { getTradesDatapoints } from '@/components/Chart/server/getTradesDatapoints';
import { getCandlesDatapoints } from '@/components/Chart/server/getCandlesDatapoints';
import { getTradesByToken } from '@/services/clickhouse/trades/trades';
import {
  CandleResolution,
  DEFAULT_RESOLUTION,
  getTradeCandlesByToken,
} from '@/services/clickhouse/trades/candles';
import { ChartTypes } from '@/components/Chart/types';
import { ResolutionSelector } from '@/components/Chart/ResolutionSelector';
import { LiveChart } from '@/components/Chart/LiveChart';

export default async function TokenPage({
  params,
  searchParams,
}: {
  params: Promise<{ tokenId: string }>;
  searchParams: Promise<{
    'resolution.value'?: CandleResolution['value'];
    'resolution.unit'?: CandleResolution['unit'];
  }>;
}) {
  const {
    'resolution.value': resolutionValue,
    'resolution.unit': resolutionUnit,
  } = await searchParams;
  const { tokenId } = await params;

  const resolution = {
    value: resolutionValue ?? DEFAULT_RESOLUTION.value,
    unit: resolutionUnit ?? DEFAULT_RESOLUTION.unit,
  };

  const trades = await getTradesByToken(tokenId);
  const candles = await getTradeCandlesByToken(tokenId, resolution);

  const tradesDatapoints = getTradesDatapoints([trades]);
  const candlesDatapoints = getCandlesDatapoints([candles]);

  const nextParams = new URLSearchParams();
  nextParams.set('resolution.value', resolution.value.toString());
  nextParams.set('resolution.unit', resolution.unit);

  return (
    <div>
      <h1>Token ID: {tokenId}</h1>
      <Link href={`/tokens?${nextParams}`}>Back to tokens list</Link>

      <LiveChart
        type={ChartTypes.Lines}
        data={tradesDatapoints}
        updateFn={async () => {
          'use server';
          const nextTrades = await getTradesByToken(tokenId);
          const nextDatapoints = getTradesDatapoints([nextTrades]);
          return nextDatapoints;
        }}
      />

      <ResolutionSelector value={resolution} />
      <LiveChart
        type={ChartTypes.Candles}
        data={candlesDatapoints}
        updateFn={async () => {
          'use server';
          const nextCandles = await getTradeCandlesByToken(tokenId, resolution);
          const nextDatapoints = getCandlesDatapoints([nextCandles]);
          return nextDatapoints;
        }}
      />
    </div>
  );
}
