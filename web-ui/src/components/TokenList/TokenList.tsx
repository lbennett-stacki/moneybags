import { Token } from '@/services/clickhouse/tokens';
import { TokenListItem } from './TokenListItem';

export const TokenList = ({
  tokens,
  nextParams,
}: {
  tokens: Token[];
  nextParams?: URLSearchParams;
}) => {
  return (
    <div className="flex flex-col gap-2">
      {tokens.map((token) => {
        return (
          <TokenListItem
            key={token.mintAddress}
            token={token}
            nextParams={nextParams}
          />
        );
      })}
    </div>
  );
};
