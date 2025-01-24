'use client';

import { Pair } from '@/services/clickhouse/pairs';
import { useEffect, useState } from 'react';
import { PairsList } from './PairsList';

export const LivePairsList = ({
  pairs,
  nextParams,
  updateFn,
}: {
  pairs: Pair[];
  nextParams?: URLSearchParams;
  updateFn: () => Promise<Pair[]>;
}) => {
  const [data, setData] = useState<Pair[]>(pairs);

  useEffect(() => {
    const interval = setInterval(async () => {
      const newData = await updateFn();
      setData(newData);
    }, 1000);

    return () => clearInterval(interval);
  }, [updateFn]);

  return <PairsList pairs={data} nextParams={nextParams} />;
};
