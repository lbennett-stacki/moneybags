import { IChartApi, LineSeries } from 'lightweight-charts';
import { RefObject, useEffect } from 'react';
import { chartTheme } from '../theme';
import { TokenListPriceData } from '../Chart';
import { roundRobin } from '@/utils/iter';

export type ChartRef = RefObject<IChartApi | null>;
export type ContainerRef = RefObject<HTMLDivElement | null>;

export const useChartData = (chartRef: ChartRef, data: TokenListPriceData) => {
  useEffect(() => {
    const chart = chartRef.current;

    if (!chart) {
      return;
    }

    for (let index = 0; index < data.length; index++) {
      const priceData = data[index];

      const newSeries = chart.addSeries(LineSeries, {
        color: roundRobin(index, chartTheme.colors.lineColors),
      });

      newSeries.setData(priceData);
    }
  }, [chartRef, data]);
};
