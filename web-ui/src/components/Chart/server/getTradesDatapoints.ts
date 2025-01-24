import { Trade } from '@/services/clickhouse/trades/trades';
import { Time } from 'lightweight-charts';

export const getTradesDatapoints = (tradesBuckets: Trade[][]) => {
  return tradesBuckets.map((trades) => {
    return trades.map((trade, i, all) => {
      const prev = all[i];

      const blockTimeModifier =
        prev && prev.blockTime.getTime() === trade.blockTime.getTime()
          ? i + 1
          : 0;

      return {
        time: (trade.blockTime.getTime() / 1000 + blockTimeModifier) as Time,
        value: Number(trade.lamportsAmount) / 1_000_000_000,
      };
    });
  });
};
