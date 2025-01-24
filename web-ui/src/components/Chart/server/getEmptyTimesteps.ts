import { Time } from 'lightweight-charts';

export const getEmptyTimesteps = <
  D extends {
    time: Time;
  },
>(
  lastDatapoint: D,
) => {
  const emptyDatapoints = [];

  const currentTime = Math.floor(Date.now() / 1000);
  const lastTime = lastDatapoint.time as number;

  for (let t = lastTime + 1; t <= currentTime; t++) {
    emptyDatapoints.push({
      ...lastDatapoint,
      time: t as Time,
    });
  }

  return emptyDatapoints;
};
