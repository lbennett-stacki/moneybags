import { pad } from '@/utils/number';
import { parseBusinessDay } from '@/utils/time';
import { isBusinessDay, Time } from 'lightweight-charts';

const formatTime = (time: Time): string => {
  const parsedTime = isBusinessDay(time)
    ? parseBusinessDay(time)
    : new Date((typeof time === 'string' ? parseInt(time, 10) : time) * 1000);

  const month = parsedTime.getMonth() + 1;
  const day = parsedTime.getDate();

  const hour = parsedTime.getHours();
  const minute = parsedTime.getMinutes();

  return `${pad(month)}/${pad(day)} ${pad(hour)}:${pad(minute)}`;
};

export default formatTime;
