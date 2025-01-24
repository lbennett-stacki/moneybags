export class Rgb {
  constructor(
    public readonly red: number,
    public readonly green: number,
    public readonly blue: number,
  ) {}

  toCssString() {
    return `rgb(${[this.red, this.green, this.blue].join(', ')})`;
  }

  static fromHex(hex: string) {
    const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
    if (!result) {
      throw new Error('Invalid hex color');
    }
    return new Rgb(
      parseInt(result[1], 16),
      parseInt(result[2], 16),
      parseInt(result[3], 16),
    );
  }
}

export class Rgba {
  constructor(
    public readonly red: number,
    public readonly green: number,
    public readonly blue: number,
    public readonly alpha: number,
  ) {}

  toCssString() {
    return `rgba(${[this.red, this.green, this.blue, this.alpha].join(', ')})`;
  }

  static fromHex(hex: string, alpha: number) {
    const rgb = Rgb.fromHex(hex);

    if (!rgb) {
      throw new Error('Invalid hex color');
    }

    return Rgba.fromRgb(rgb, alpha);
  }

  static fromRgb(rgb: Rgb, alpha: number) {
    return new Rgba(rgb.red, rgb.green, rgb.blue, alpha);
  }
}
