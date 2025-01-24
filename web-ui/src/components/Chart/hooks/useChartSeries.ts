'use client';

import {
  CandlestickSeries,
  HistogramSeries,
  IChartApi,
  ISeriesApi,
  LineData,
  LineSeries,
  SeriesType,
} from 'lightweight-charts';
import { RefObject, useEffect } from 'react';
import { chartTheme } from '../theme';
import { roundRobin } from '@/utils/iter';
import { ChartTypes } from '../types';
import { CandleData, ChartData } from '../props';
import colors from 'tailwindcss/colors';
import { Rgba } from '@/utils/rgb';

export type ChartRef = RefObject<IChartApi | null>;
export type ContainerRef = RefObject<HTMLDivElement | null>;

export const useChartSeries = <T extends ChartTypes, D extends ChartData>(
  type: T,
  chartRef: ChartRef,
  inputData: D,
  seriesRef: RefObject<ISeriesApi<SeriesType> | null>,
) => {
  useEffect(() => {
    const chart = chartRef.current;

    if (!chart) {
      return;
    }

    const series: ISeriesApi<SeriesType>[] = [];

    const data =
      type === ChartTypes.Lines
        ? (inputData as LineData[][])
        : (inputData as CandleData[]).map((item) => item.candles);

    const volumes =
      type === ChartTypes.Lines
        ? []
        : (inputData as CandleData[]).map((item) => item.volumes);

    for (let index = 0; index < data.length; index++) {
      const priceData = data[index];

      const dataSeries = chart.addSeries(
        type === 'lines' ? LineSeries : CandlestickSeries,
        {
          color: roundRobin(index, chartTheme.colors.line),
        },
      );
      dataSeries.priceScale().applyOptions({
        scaleMargins: {
          top: 0.1,
          bottom: 0.4,
        },
      });

      dataSeries.setData(priceData);
      series.push(dataSeries);

      if (index === 0) {
        seriesRef.current = dataSeries;
      }
    }

    for (let index = 0; index < volumes.length; index++) {
      const volumeData = volumes[index];

      const volumeSeries = chart.addSeries(HistogramSeries, {
        color: Rgba.fromHex(colors.blue[500], 0.3).toCssString(),
        priceScaleId: '',
        priceFormat: {
          type: 'volume',
        },
      });
      volumeSeries.priceScale().applyOptions({
        scaleMargins: {
          top: 0.7,
          bottom: 0,
        },
      });

      series.push(volumeSeries);
      volumeSeries.setData(volumeData);
    }

    return () => {
      series.forEach((series) => {
        try {
          chart.removeSeries(series);
          // eslint-disable-next-line @typescript-eslint/no-unused-vars
        } catch (error) {
          return;
        }
      });
      seriesRef.current = null;
    };
  }, [chartRef, inputData, type, seriesRef]);
};
