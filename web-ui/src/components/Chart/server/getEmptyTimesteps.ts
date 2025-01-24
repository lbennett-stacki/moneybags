import { CandleResolution } from '@/services/clickhouse/trades/candles';
import { Time } from 'lightweight-charts';

export const getEmptyTimesteps = <
  D extends {
    time: Time;
  },
>(
  lastDatapoint: D,
  resolution: CandleResolution,
  maxCandles: number = 10,
) => {
  const emptyDatapoints = [];

  const lastTime = lastDatapoint.time as number;
  const currentTime = Math.floor(Date.now() / 1000);

  const resolutionInSeconds =
    resolution.unit === 'second'
      ? resolution.value
      : resolution.unit === 'minute'
        ? resolution.value * 60
        : resolution.unit === 'hour'
          ? resolution.value * 60 * 60
          : resolution.unit === 'day'
            ? resolution.value * 60 * 60 * 24
            : resolution.value;

  const boundedTime = Math.min(
    currentTime,
    lastTime + maxCandles * resolutionInSeconds,
  );

  const diff = boundedTime - lastTime;
  const numSteps = Math.ceil(diff / resolutionInSeconds);

  for (let i = 1; i <= numSteps; i++) {
    const t = lastTime + i * resolutionInSeconds;

    emptyDatapoints.push({
      ...lastDatapoint,
      time: t as Time,
    });
  }

  return emptyDatapoints;
};
