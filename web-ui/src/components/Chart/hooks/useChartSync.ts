import {
  IChartApi,
  ISeriesApi,
  LogicalRange,
  MouseEventParams,
  SeriesType,
  Time,
} from 'lightweight-charts';
import { RefObject, useEffect } from 'react';

type DataPoint = { time: Time; value: number };

export const useChartSync = (
  chartRef: RefObject<IChartApi | null>,
  seriesRef: RefObject<ISeriesApi<SeriesType> | null>,
  syncWithChart?: RefObject<IChartApi | null>,
  syncWithSeries?: RefObject<ISeriesApi<SeriesType> | null>,
) => {
  useEffect(() => {
    const chart = chartRef.current;
    const syncChart = syncWithChart?.current;
    const series = seriesRef.current;
    const syncSeries = syncWithSeries?.current;

    if (!chart || !syncChart || !series || !syncSeries) {
      return;
    }

    chart
      .timeScale()
      .subscribeVisibleLogicalRangeChange((timeRange: LogicalRange | null) => {
        if (timeRange !== null) {
          syncChart.timeScale().setVisibleLogicalRange(timeRange);
        }
      });

    const getCrosshairDataPoint = (
      series: ISeriesApi<SeriesType>,
      param: MouseEventParams<Time>,
    ) => {
      if (!param.time || !param.seriesData) {
        return null;
      }
      const dataPoint = param.seriesData.get(series) as DataPoint;
      return dataPoint || null;
    };

    const syncCrosshair = (
      chart: IChartApi,
      series: ISeriesApi<SeriesType>,
      dataPoint: DataPoint | null,
    ) => {
      if (dataPoint) {
        chart.setCrosshairPosition(dataPoint.value, dataPoint.time, series);
        return;
      }
      chart.clearCrosshairPosition();
    };

    chart.subscribeCrosshairMove((param: MouseEventParams<Time>) => {
      const dataPoint = getCrosshairDataPoint(series, param);
      syncCrosshair(syncChart, syncSeries, dataPoint);
    });
  }, [chartRef, seriesRef, syncWithChart, syncWithSeries]);
};
