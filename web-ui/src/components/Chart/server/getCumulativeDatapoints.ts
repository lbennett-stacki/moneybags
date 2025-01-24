import { LineData, Time } from 'lightweight-charts';
import { getEmptyTimesteps } from './getEmptyTimesteps';
import { CandleResolution } from '@/services/clickhouse/trades/candles';

export const getCumulativeDatapoints = (
  datapointBuckets: LineData[][],
  resolution: CandleResolution,
) => {
  return datapointBuckets.map((datapoints) => {
    const existingDatapoints = datapoints.map((datapoint, i, all) => {
      const prev = all[i];

      const blockTimeModifier =
        prev && prev.time === datapoint.time ? i + 1 : 0;

      return {
        time: ((datapoint.time as number) + blockTimeModifier) as Time,
        value: datapoint.value,
      };
    });

    const lastDatapoint = existingDatapoints[existingDatapoints.length - 1]!;

    const emptyDatapoints = getEmptyTimesteps(lastDatapoint, resolution).map(
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
