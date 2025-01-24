import { LiveChart } from '@/components/Chart/LiveChart';
import { getCumulativeDatapoints } from '@/components/Chart/server/getCumulativeDatapoints';
import { ChartTypes } from '@/components/Chart/types';
import { LiveTokenList } from '@/components/TokenList/LiveTokenList';
import { getTokenList } from '@/services/clickhouse/tokens';
import {
  getCumulativeTokenCount,
  getCumulativeTradeCount,
} from '@/services/clickhouse/trades/cumulative';
import { Time } from 'lightweight-charts';

export default async function TokensPage() {
  const tokens = await getTokenList();
  const tokensCount = await getCumulativeTokenCount();
  const tradesCount = await getCumulativeTradeCount();

  const tokensData = getCumulativeDatapoints([
    tokensCount.map((token) => ({
      time: (token.time_bucket.getTime() / 1000) as Time,
      value: token.count,
    })),
  ]);
  const tradesData = getCumulativeDatapoints([
    tradesCount.map((trade) => ({
      time: (trade.time_bucket.getTime() / 1000) as Time,
      value: trade.count,
    })),
  ]);

  const data = [tokensData[0]!, tradesData[0]!];

  return (
    <div>
      <h1>Tokens Page</h1>

      <LiveChart
        type={ChartTypes.Lines}
        data={data}
        updateFn={async () => {
          'use server';
          const nextTokensCount = await getCumulativeTokenCount();
          const nextTradesCount = await getCumulativeTradeCount();

          const nextTokensData = getCumulativeDatapoints([
            nextTokensCount.map((token) => ({
              time: (token.time_bucket.getTime() / 1000) as Time,
              value: token.count,
            })),
          ]);
          const nextTradesData = getCumulativeDatapoints([
            nextTradesCount.map((trade) => ({
              time: (trade.time_bucket.getTime() / 1000) as Time,
              value: trade.count,
            })),
          ]);
          const nextData = [nextTokensData[0]!, nextTradesData[0]!];

          return nextData;
        }}
      />

      <LiveTokenList
        tokens={tokens}
        updateFn={async () => {
          'use server';
          const nextTokens = await getTokenList();
          return nextTokens;
        }}
      />
    </div>
  );
}
