import { createClient } from '@clickhouse/client';
import { z } from 'zod';

export const client = createClient({
  url: 'http://localhost:8123',
  username: 'default',
  password: '',
  database: 'moneybags',
});

const tokenSchema = z
  .object({
    mint: z.string(),
    bonding_curve: z.string(),
    inserted_at: z.string(),
  })
  .transform(
    ({ bonding_curve: bondingCurve, inserted_at: insertedAt, ...rest }) => {
      return {
        ...rest,
        bondingCurve,
        insertedAt: new Date(insertedAt),
      };
    },
  );

const tokenListQueryResult = z.object({
  data: z.array(tokenSchema),
});

export const getTokenList = async () => {
  const result = await client.query({
    query: 'SELECT * FROM tokens',
  });

  const json = await result.json();

  return tokenListQueryResult.parse(json).data;
};
