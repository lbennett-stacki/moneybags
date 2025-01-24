import { shuffle } from '@/utils/iter';
import { Rgba } from '@/utils/rgb';
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
  .flatMap(([, shades]: [string, Record<string, string>]) => {
    return Object.entries(shades)
      .filter(([name]) => {
        const int = parseInt(name);

        return int <= 800 && int >= 300;
      })
      .map(([, shade]) => shade);
  });

export const chartTheme = {
  colors: {
    background: '#0A0A0A',
    text: '#EDEDED',
    line: shuffle(colorfulShades),
    grid: Rgba.fromHex('#FFFFFF', 0.1).toCssString(),
  },
};
