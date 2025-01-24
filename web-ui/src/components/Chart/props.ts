import { CandlestickData, LineData } from 'lightweight-charts';
import { ChartTypes } from './types';

export interface CandleData {
  candles: CandlestickData[];
  volumes: LineData[];
}

export type ChartData = LineData[][] | CandleData[];

export interface ChartProps<T extends ChartTypes, D extends ChartData> {
  type: T;
  data: D;
}
