import { Token } from '@/services/clickhouse/tokens';
import Link from 'next/link';

export const TokenListItem = ({
  token,
  nextParams,
}: {
  token: Token;
  nextParams?: URLSearchParams;
}) => {
  const params = nextParams ? `?${nextParams}` : '';
  return (
    <Link
      key={token.mintAddress}
      href={`/tokens/${token.mintAddress}${params}`}
    >
      {`${token.mintAddress} [${token.isComplete ? 'COMPLETE' : 'INCOMPLETE'}]  (${token.tradesCount} trades) <${token.price}>`}
    </Link>
  );
};
