import { shuffle } from '@/utils/iter';
import tailwindColors from 'tailwindcss/colors';

const colorfulShades: string[] = Object.entries(tailwindColors)
  .filter(
    ([name, shades]) =>
      typeof shades === 'object' &&
      '500' in shades &&
      ![
        'gray',
        'slate',
        'stone',
        'neutral',
        'zinc',
        'blueGray',
        'warmGray',
        'trueGray',
      ].includes(name),
  )
  .flatMap(([_, shades]: [string, Record<string, string>]) => {
    return Object.entries(shades)
      .filter(([name, _]) => {
        const int = parseInt(name);

        return int <= 800 && int >= 300;
      })
      .map(([_, shade]) => shade);
  });

export const chartTheme = {
  colors: {
    backgroundColor: '#0a0a0a',
    textColor: '#ededed',
    lineColors: shuffle(colorfulShades),
  },
};
