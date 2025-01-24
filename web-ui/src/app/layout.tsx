import type { Metadata } from 'next';
import { Quicksand } from 'next/font/google';
import './globals.css';

const font = Quicksand({
  variable: '--font-quicksand',
  subsets: ['latin'],
});

export const metadata: Metadata = {
  title: 'moneybags',
  description: 'moneybags',
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={`${font.variable}  antialiased`}>{children}</body>
    </html>
  );
}
