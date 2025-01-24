'use client';

import { useEffect } from 'react';
import { ChartRef, ContainerRef } from './useChart';

export const useResizableChart = (
  containerRef: ContainerRef,
  chartRef: ChartRef,
  maxHeight?: number,
) => {
  useEffect(() => {
    const chart = chartRef.current;
    const container = containerRef.current;

    if (!chart || !container) {
      return;
    }

    const handleResize = () => {
      chart.applyOptions({
        width: container.clientWidth,
        height: Math.min(container.clientHeight, maxHeight ?? Infinity),
      });
    };

    chart.timeScale().fitContent();

    window.addEventListener('resize', handleResize);
    handleResize();

    return () => {
      window.removeEventListener('resize', handleResize);
    };
  }, [chartRef, containerRef, maxHeight]);
};
