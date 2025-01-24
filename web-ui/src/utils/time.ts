import { BusinessDay } from 'lightweight-charts';

export const parseBusinessDay = (day: BusinessDay): Date => {
  const date = new Date();
  date.setFullYear(day.year);
  date.setMonth(day.month - 1);
  date.setDate(day.day);

  return date;
};
