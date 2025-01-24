'use client';

import { useChart } from './hooks/useChart';
import { useResizableChart } from './hooks/useResizableChart';
import { useChartSeries } from './hooks/useChartSeries';
import { ChartTypes } from './types';
import { ChartData, ChartProps } from './props';

export const Chart = <T extends ChartTypes, D extends ChartData>({
  type,
  data,
}: ChartProps<T, D>) => {
  const { container, chart } = useChart(type);

  useResizableChart(container, chart);
  useChartSeries(type, chart, data);

  return <div ref={container} />;
};
