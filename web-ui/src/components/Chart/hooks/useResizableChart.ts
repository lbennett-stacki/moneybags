import { useEffect } from 'react';
import { ChartRef, ContainerRef } from './useChart';

export const useResizableChart = (
  containerRef: ContainerRef,
  chartRef: ChartRef,
) => {
  useEffect(() => {
    const chart = chartRef.current;
    const container = containerRef.current;

    if (!chart || !container) {
      return;
    }

    const handleResize = () => {
      chart.applyOptions({ width: container.clientWidth });
    };

    chart.timeScale().fitContent();

    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);

      chart.remove();
    };
  }, [chartRef, containerRef]);
};
