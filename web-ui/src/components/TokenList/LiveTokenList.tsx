'use client';

import { Token } from '@/services/clickhouse/tokens';
import { useEffect, useState } from 'react';
import { TokenList } from './TokenList';

export const LiveTokenList = ({
  tokens,
  nextParams,
  updateFn,
}: {
  tokens: Token[];
  nextParams?: URLSearchParams;
  updateFn: () => Promise<Token[]>;
}) => {
  const [data, setData] = useState<Token[]>(tokens);

  useEffect(() => {
    const interval = setInterval(async () => {
      const newData = await updateFn();
      setData(newData);
    }, 1000);

    return () => clearInterval(interval);
  }, [updateFn]);

  return <TokenList tokens={data} nextParams={nextParams} />;
};
