import type { Metadata } from 'next';
import { ReactNode } from 'react';
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
  children: ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={`${font.variable}  antialiased w-full h-screen`}>
        {children}
      </body>
    </html>
  );
}
