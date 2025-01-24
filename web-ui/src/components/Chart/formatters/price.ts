import { ChartTypes } from '../types';

export const formatPrice = (price: number, type: ChartTypes): string => {
  const lamports = type === ChartTypes.Candles ? price * 1_000_000_000 : price;

  const units = [
    { threshold: 1_000_000_000, symbol: 'SOL', divisor: 1_000_000_000 },
    { threshold: 1_000_000, symbol: 'MΛ', divisor: 1_000_000 },
    { threshold: 1_000, symbol: 'kΛ', divisor: 1_000 },
    { threshold: 1, symbol: 'Λ', divisor: 1 },
    { threshold: 0.001, symbol: 'mΛ', divisor: 0.001 },
    { threshold: 0.000001, symbol: 'μΛ', divisor: 0.000001 },
  ];

  const unit =
    units.find((u) => lamports >= u.threshold) || units[units.length - 1];

  const value = lamports / unit.divisor;
  const displayValue =
    value >= 1000 ? value.toExponential(2) : value.toPrecision(3);

  return `${displayValue} ${unit.symbol}`;
};
