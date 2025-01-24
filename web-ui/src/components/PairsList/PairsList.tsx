import { Pair } from '@/services/clickhouse/pairs';
import { PairsListItem } from './PairsListItem';

export const PairsList = ({
  pairs,
  nextParams,
}: {
  pairs: Pair[];
  nextParams?: URLSearchParams;
}) => {
  return (
    <div className="flex flex-col gap-2">
      {pairs.map((pair) => {
        return (
          <PairsListItem
            key={pair.pairKey}
            pair={pair}
            nextParams={nextParams}
          />
        );
      })}
    </div>
  );
};
