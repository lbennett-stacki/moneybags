'use client';

import { usePathname, useRouter, useSearchParams } from 'next/navigation';
import {
  CandleResolution,
  candleResolutionUnits,
  DEFAULT_RESOLUTION,
  isCandleResolutionUnit,
  isCandleResolutionValue,
} from '@/services/clickhouse/trades/candles';

interface ResolutionSelectorProps {
  value?: CandleResolution;
}

export function ResolutionSelector({ value }: ResolutionSelectorProps) {
  const router = useRouter();
  const pathname = usePathname();
  const searchParams = useSearchParams();

  const updateResolutionValue = (value: CandleResolution['value']) => {
    const params = new URLSearchParams(searchParams.toString());

    params.set('resolution.value', value.toString());

    router.push(`${pathname}?${params.toString()}`);
  };

  const updateResolutionUnit = (unit: CandleResolution['unit']) => {
    const params = new URLSearchParams(searchParams.toString());

    params.set('resolution.unit', unit);

    router.push(`${pathname}?${params.toString()}`);
  };

  return (
    <div>
      <input
        type="number"
        value={value?.value}
        min={1}
        onChange={(e) => {
          const value = Number(e.target.value);

          if (!isCandleResolutionValue(value)) {
            return;
          }

          updateResolutionValue(value);
        }}
      />
      <select
        value={value?.unit ?? DEFAULT_RESOLUTION.unit}
        onChange={(e) => {
          if (!isCandleResolutionUnit(e.target.value)) {
            return;
          }

          updateResolutionUnit(e.target.value);
        }}
      >
        {candleResolutionUnits.map((candleResolution) => (
          <option key={candleResolution} value={candleResolution}>
            {candleResolution}
          </option>
        ))}
      </select>
    </div>
  );
}
