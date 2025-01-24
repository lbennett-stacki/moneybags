import { createClient } from '@clickhouse/client';

export const client = createClient({
  url: 'http://localhost:8123',
  username: 'default',
  password: '',
  database: 'moneybags',
});
