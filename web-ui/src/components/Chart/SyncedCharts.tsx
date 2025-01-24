'use client';

import { RefObject, useRef, useState, useEffect } from 'react';
import { LiveChart } from './LiveChart';
import { ChartRefs } from './Chart';
import { ChartTypes } from './types';
import { Time } from 'lightweight-charts';
import { ChartData, ChartProps } from './props';

interface ChartConfig {
  height: number | string;
  data: { time: Time; value: number }[][];
  onUpdate: () => Promise<{ time: Time; value: number }[][]>;
}

interface SyncedChartsProps {
  charts: ChartConfig[];
}

export interface SyncableChartProps<T extends ChartTypes, D extends ChartData>
  extends ChartProps<T, D> {
  syncWith?: ChartRefs;
}

export type ChartRefObject = RefObject<ChartRefs>;

export const useChartRef = (): ChartRefObject => {
  return useRef<ChartRefs>({
    chartRef: { current: null },
    seriesRef: { current: null },
  });
};

export const useSyncableChartsReady = (chartRefs: ChartRefObject[]) => {
  const [chartsReady, setChartsReady] = useState(false);

  useEffect(() => {
    const allChartsReady = chartRefs.every(
      (ref) => ref.current?.chartRef.current && ref.current?.seriesRef.current,
    );

    if (allChartsReady) {
      setChartsReady(true);
    }
  }, [chartRefs]);

  return chartsReady;
};

export const SyncedCharts = ({ charts }: SyncedChartsProps) => {
  const chartRefs = useRef(
    charts.map(() => ({
      current: {
        chartRef: { current: null },
        seriesRef: { current: null },
      },
    })),
  );
  const chartsReady = useSyncableChartsReady(chartRefs.current);

  return (
    <>
      {charts.map((chart, index) => (
        <LiveChart
          key={index}
          type={ChartTypes.Lines}
          data={chart.data}
          updateFn={chart.onUpdate}
          height={chart.height}
          ref={chartRefs.current[index]}
          syncWith={
            chartsReady
              ? {
                  chartRef:
                    chartRefs.current[(index + 1) % charts.length].current!
                      .chartRef,
                  seriesRef:
                    chartRefs.current[(index + 1) % charts.length].current!
                      .seriesRef,
                }
              : undefined
          }
        />
      ))}
    </>
  );
};
