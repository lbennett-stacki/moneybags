import { CandlestickData, LineData } from 'lightweight-charts';

export enum ChartTypes {
  Lines = 'lines',
  Candles = 'candles',
}

export interface ChartProps<
  T extends ChartTypes,
  D extends LineData[][] | CandlestickData[][],
> {
  type: T;
  data: D;
}
