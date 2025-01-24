'use client';

import { useChart } from './hooks/useChart';
import { useResizableChart } from './hooks/useResizableChart';
import { useChartSeries } from './hooks/useChartSeries';
import { useChartSync } from './hooks/useChartSync';
import { ChartTypes } from './types';
import { ChartData } from './props';
import { IChartApi, ISeriesApi, SeriesType } from 'lightweight-charts';
import { forwardRef, useImperativeHandle, RefObject } from 'react';
import { SyncableChartProps } from './SyncedCharts';

export interface ChartRefs {
  chartRef: RefObject<IChartApi | null>;
  seriesRef: RefObject<ISeriesApi<SeriesType> | null>;
}

export interface ChartProps<T extends ChartTypes, D extends ChartData>
  extends SyncableChartProps<T, D> {
  height?: number;
  className?: string;
}

export const Chart = forwardRef<ChartRefs, ChartProps<ChartTypes, ChartData>>(
  ({ type, data, syncWith, height, className }, ref) => {
    const { container, chart, series } = useChart(type, height);

    useResizableChart(container, chart, height);
    useChartSeries(type, chart, data, series);
    useChartSync(chart, series, syncWith?.chartRef, syncWith?.seriesRef);

    useImperativeHandle(ref, () => ({
      chartRef: chart,
      seriesRef: series,
    }));

    return (
      <div
        ref={container}
        className={['h-full w-full relative', className]
          .filter(Boolean)
          .join(' ')}
        style={{ height }}
      />
    );
  },
);

Chart.displayName = 'Chart';
