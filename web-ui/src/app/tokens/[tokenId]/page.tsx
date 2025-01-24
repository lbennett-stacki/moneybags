import { Chart } from '@/components/Chart/Chart';
import { getTradesDatapoints } from '@/components/Chart/server/getTradesDatapoints';
import { getCandlesDatapoints } from '@/components/Chart/server/getCandlesDatapoints';
import { getPumpFunTradesByToken } from '@/services/clickhouse/trades/trades';
import {
  CandleResolution,
  DEFAULT_RESOLUTION,
  getPumpFunTradeCandlesByToken,
} from '@/services/clickhouse/trades/candles';
import { ChartTypes } from '@/components/Chart/types';
import { ResolutionSelector } from '@/components/Chart/ResolutionSelector';
import Link from 'next/link';

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

  const trades = await getPumpFunTradesByToken(tokenId);
  const candles = await getPumpFunTradeCandlesByToken(tokenId, resolution);

  const tradesDatapoints = getTradesDatapoints([trades]);
  const candlesDatapoints = getCandlesDatapoints([candles]);

  return (
    <div>
      <h1>Token ID: {tokenId}</h1>
      <Link href="/tokens">Back to tokens list</Link>

      <Chart type={ChartTypes.Lines} data={tradesDatapoints} />

      <ResolutionSelector value={resolution} />
      <Chart type={ChartTypes.Candles} data={candlesDatapoints} />
    </div>
  );
}
