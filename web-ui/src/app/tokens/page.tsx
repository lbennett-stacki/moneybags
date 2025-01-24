import { getTokenList } from '@/services/clickhouse';
import Link from 'next/link';

export default async function TokensPage() {
  const tokens = await getTokenList();

  return (
    <div>
      <h1>Tokens Page</h1>

      {tokens.map((token) => {
        return (
          <Link key={token.mint} href={`/tokens/${token.mint}`}>
            Go to Token {JSON.stringify(token)}
          </Link>
        );
      })}
    </div>
  );
}
