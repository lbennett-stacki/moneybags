'use client';

import {
  ColorType,
  createChart,
  IChartApi,
  ISeriesApi,
  SeriesType,
  Time,
} from 'lightweight-charts';
import { RefObject, useEffect, useRef } from 'react';
import { chartTheme } from '../theme';
import { ChartTypes } from '../types';
import { formatPrice } from '../formatters/price';
import formatTime from '../formatters/time';

export type ChartRef = RefObject<IChartApi | null>;
export type ContainerRef = RefObject<HTMLDivElement | null>;
export type SeriesRef = RefObject<ISeriesApi<SeriesType> | null>;

export const useChart = (type: ChartTypes, height?: number) => {
  const containerRef: ContainerRef = useRef(null);
  const chartRef: ChartRef = useRef(null);
  const seriesRef: SeriesRef = useRef(null);

  useEffect(() => {
    const container = containerRef.current;

    if (!container || chartRef.current) {
      return;
    }

    const chart = createChart(container, {
      layout: {
        background: {
          type: ColorType.Solid,
          color: chartTheme.colors.background,
        },
        textColor: chartTheme.colors.text,
        attributionLogo: false,
      },
      width: container.clientWidth,
      grid: {
        horzLines: {
          color: chartTheme.colors.grid,
        },
        vertLines: {
          color: chartTheme.colors.grid,
        },
      },
      height,
      localization: {
        priceFormatter: (price: number) => {
          return formatPrice(price, type);
        },
      },
      timeScale: {
        tickMarkFormatter: (time: Time) => {
          return formatTime(time);
        },
      },
    });

    chartRef.current = chart;

    return () => {
      chart.remove();
      chartRef.current = null;
      containerRef.current = null;
      seriesRef.current = null;
    };
  }, [containerRef, chartRef, type, height]);

  return { container: containerRef, chart: chartRef, series: seriesRef };
};
