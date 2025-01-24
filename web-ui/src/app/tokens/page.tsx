import { Chart } from '@/components/Chart/Chart';
import { ResolutionSelector } from '@/components/Chart/ResolutionSelector';
import { getCandlesDatapoints } from '@/components/Chart/server/getCandlesDatapoints';
import { getTradesDatapoints } from '@/components/Chart/server/getTradesDatapoints';
import { ChartTypes } from '@/components/Chart/types';
import { getTokenList } from '@/services/clickhouse/tokens';
import {
  CandleResolution,
  DEFAULT_RESOLUTION,
  getAllPumpFunTradeCandles,
} from '@/services/clickhouse/trades/candles';
import { getAllPumpFunTrades } from '@/services/clickhouse/trades/trades';
import Link from 'next/link';

const groupByToken = <T extends { mintAddress: string }>(items: T[]) => {
  return items.reduce(
    (acc, item) => {
      acc[item.mintAddress] ??= [];
      acc[item.mintAddress].push(item);
      return acc;
    },
    {} as Record<string, T[]>,
  );
};

export default async function TokensPage({
  searchParams,
}: {
  searchParams: Promise<{
    'resolution.unit'?: CandleResolution['unit'];
    'resolution.value'?: CandleResolution['value'];
  }>;
}) {
  const {
    'resolution.unit': resolutionUnit,
    'resolution.value': resolutionValue,
  } = await searchParams;

  const resolution = {
    value: resolutionValue ?? DEFAULT_RESOLUTION.value,
    unit: resolutionUnit ?? DEFAULT_RESOLUTION.unit,
  };

  const tokens = await getTokenList();
  const allTrades = await getAllPumpFunTrades();
  const allCandles = await getAllPumpFunTradeCandles(resolution);

  const tradesByToken = groupByToken(allTrades);
  const candlesByToken = groupByToken(allCandles);

  const tradesData = getTradesDatapoints(Object.values(tradesByToken));
  const candlesData = getCandlesDatapoints(Object.values(candlesByToken));

  return (
    <div>
      <h1>Tokens Page</h1>

      <Chart type={ChartTypes.Lines} data={tradesData} />

      <ResolutionSelector value={resolution} />
      <Chart type={ChartTypes.Candles} data={candlesData} />

      <div className="flex flex-col gap-2">
        {tokens.map((token) => {
          return (
            <Link key={token.mintAddress} href={`/tokens/${token.mintAddress}`}>
              {`${token.mintAddress} [${token.isComplete ? 'COMPLETE' : 'INCOMPLETE'}]  (${token.tradesCount} trades) <${token.price}>`}
            </Link>
          );
        })}
      </div>
    </div>
  );
}
