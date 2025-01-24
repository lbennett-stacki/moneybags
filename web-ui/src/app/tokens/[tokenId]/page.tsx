import { Chart } from '@/components/Chart/Chart';
import { Time } from 'lightweight-charts';

const generateRandomTimeSeries = () => {
  const startDate = new Date('2024-01-01');
  const days = 365;
  const baseValue = 100 * 50;
  const volatility = 0.3;

  return Array.from({ length: days }, (_, i) => {
    const date = new Date(startDate);
    date.setDate(startDate.getDate() + i);

    const randomChange = (Math.random() - 0.5) * volatility;
    const value = baseValue * (1 + randomChange) * (1 + Math.sin(i / 3) * 0.5);

    return {
      time: date.getTime() as Time,
      value: Math.round(value),
    };
  });
};

export default async function TokenPage({
  params,
}: {
  params: Promise<{ tokenId: string }>;
}) {
  const { tokenId } = await params;

  const data = Array.from({ length: 10 }, (_, i) => generateRandomTimeSeries());

  return (
    <div>
      <h1>Token ID: {tokenId}</h1>

      <Chart data={data} />
    </div>
  );
}
