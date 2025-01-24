'use client';

import { Time } from 'lightweight-charts';
import { useChart } from './hooks/useChart';
import { useResizableChart } from './hooks/useResizableChart';
import { useChartData } from './hooks/useChartData';

export interface PriceDatapoint {
  time: Time;
  value: number;
}
export type TokenPriceData = PriceDatapoint[];
export type TokenListPriceData = TokenPriceData[];

export const Chart = ({ data }: { data: TokenListPriceData }) => {
  const { container, chart } = useChart();

  useResizableChart(container, chart);
  useChartData(chart, data);

  return <div ref={container} />;
};
