import { Trade } from '@/services/clickhouse/trades/trades';
import { Time } from 'lightweight-charts';
import { getEmptyTimesteps } from './getEmptyTimesteps';

export const getTradesDatapoints = (tradesBuckets: Trade[][]) => {
  return tradesBuckets.map((trades) => {
    const existingDatapoints = trades.map((trade, i, all) => {
      const prev = all[i];

      const blockTimeModifier =
        prev && prev.blockTime.getTime() === trade.blockTime.getTime()
          ? i + 1
          : 0;

      return {
        time: (trade.blockTime.getTime() / 1000 + blockTimeModifier) as Time,
        value: Number(trade.coinTokenAmount / trade.priceCoinTokenAmount),
      };
    });

    const lastDatapoint = existingDatapoints[existingDatapoints.length - 1]!;

    const emptyDatapoints = getEmptyTimesteps(lastDatapoint).map(
      (datapoint) => {
        return {
          ...datapoint,
          value: lastDatapoint.value,
        };
      },
    );

    return [...existingDatapoints, ...emptyDatapoints];
  });
};
