'use client';

import { useEffect, useState } from 'react';
import { ChartTypes } from './types';
import { ChartData, ChartProps } from './props';
import { Chart } from './Chart';

export const LiveChart = <T extends ChartTypes, D extends ChartData>({
  type,
  data: initialData,
  updateFn,
}: ChartProps<T, D> & { updateFn: () => Promise<D> }) => {
  const [data, setData] = useState<D>(initialData);

  useEffect(() => {
    const interval = setInterval(async () => {
      const newData = await updateFn();
      setData(newData);
    }, 1000);

    return () => clearInterval(interval);
  }, [updateFn]);

  return <Chart type={type} data={data} />;
};
