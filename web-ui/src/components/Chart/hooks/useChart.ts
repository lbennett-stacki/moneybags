import {
  BusinessDay,
  ColorType,
  createChart,
  IChartApi,
  isBusinessDay,
  Time,
} from 'lightweight-charts';
import { RefObject, useEffect, useRef } from 'react';
import { chartTheme } from '../theme';

export type ChartRef = RefObject<IChartApi | null>;
export type ContainerRef = RefObject<HTMLDivElement | null>;

const parseBusinessDay = (day: BusinessDay): Date => {
  const date = new Date();
  date.setFullYear(day.year);
  date.setMonth(day.month - 1);
  date.setDate(day.day);

  return date;
};

const pad = (value: number) => {
  return value.toString().padStart(2, '0');
};

export const useChart = () => {
  const containerRef: ContainerRef = useRef(null);
  const chartRef: ChartRef = useRef(null);

  useEffect(() => {
    const container = containerRef.current;

    if (!container || chartRef.current) {
      return;
    }

    const chart = createChart(container, {
      layout: {
        background: {
          type: ColorType.Solid,
          color: chartTheme.colors.backgroundColor,
        },
        textColor: chartTheme.colors.textColor,
      },
      width: container.clientWidth,
      height: 300,
      timeScale: {
        tickMarkFormatter: (time: Time) => {
          const parsedTime = isBusinessDay(time)
            ? parseBusinessDay(time)
            : new Date(time);

          const month = parsedTime.getMonth() + 1;
          const day = parsedTime.getDate();

          const hour = parsedTime.getHours();
          const minute = parsedTime.getMinutes();

          return `${pad(month)}/${pad(day)} ${pad(hour)}:${pad(minute)}`;
        },
      },
    });

    chartRef.current = chart;

    return () => {
      chart.remove();
    };
  }, [containerRef, chartRef]);

  return { container: containerRef, chart: chartRef };
};
