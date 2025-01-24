import { Pair } from '@/services/clickhouse/pairs';
import Link from 'next/link';

export const PairsListItem = ({
  pair,
  nextParams,
}: {
  pair: Pair;
  nextParams?: URLSearchParams;
}) => {
  const params = nextParams ? `?${nextParams}` : '';

  return (
    <Link key={pair.pairKey} href={`/pairs/${pair.pairKey}${params}`}>
      {`${pair.coinTokenAddress} / ${pair.priceCoinTokenAddress} (${pair.tradesCount} trades)`}
    </Link>
  );
};
