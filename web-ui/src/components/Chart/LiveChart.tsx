'use client';

import { useEffect, useState, forwardRef } from 'react';
import { ChartTypes } from './types';
import { ChartData } from './props';
import { Chart, ChartRefs } from './Chart';
import { SyncableChartProps } from './SyncedCharts';

export interface LiveChartProps<T extends ChartTypes, D extends ChartData>
  extends SyncableChartProps<T, D> {
  updateFn: () => Promise<D>;
  height?: number;
  className?: string;
}

export const LiveChart = forwardRef<
  ChartRefs,
  LiveChartProps<ChartTypes, ChartData>
>(({ type, data: initialData, updateFn, syncWith, height, className }, ref) => {
  const [data, setData] = useState<ChartData>(initialData);

  useEffect(() => {
    const interval = setInterval(async () => {
      const newData = await updateFn();
      setData(newData);
    }, 1000);

    return () => clearInterval(interval);
  }, [updateFn]);

  return (
    <Chart
      className={className}
      type={type}
      data={data}
      ref={ref}
      syncWith={syncWith}
      height={height}
    />
  );
});

LiveChart.displayName = 'LiveChart';
