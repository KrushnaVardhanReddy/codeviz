import NextAuth from 'next-auth';
import { authConfig } from './auth.config';
import { SurrealDBAdapter } from '@auth/surrealdb-adapter';
import { clientPromise } from './lib/surrealdb';

export const { handlers, auth, signIn, signOut } = NextAuth({
  ...authConfig,
  adapter: SurrealDBAdapter(clientPromise),
});
