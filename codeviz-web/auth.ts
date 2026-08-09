import NextAuth from "next-auth"
import GitHub from "next-auth/providers/github"
import Google from "next-auth/providers/google"
import Credentials from "next-auth/providers/credentials"
import { SupabaseAdapter } from "@auth/supabase-adapter"

const isE2E = process.env.E2E_TEST === 'true';

const supabaseUrl = process.env.SUPABASE_URL || 'http://localhost:54321';
const supabaseKey = process.env.SUPABASE_SERVICE_ROLE_KEY || 'dummy_key';

const providers: any[] = [
  GitHub,
  Google
];

if (isE2E) {
  providers.push(
    Credentials({
      name: "E2E Test Account",
      credentials: {
        username: { label: "Username", type: "text", placeholder: "testuser" },
        password: { label: "Password", type: "password" }
      },
      async authorize(credentials) {
        if (credentials?.username === "testuser" && credentials?.password === "password") {
          return { id: "1", name: "Test User", email: "testuser@example.com" };
        }
        return null;
      }
    })
  );
}

export const { handlers, auth, signIn, signOut } = NextAuth({
  secret: isE2E ? "dummy_secret_for_tests" : process.env.AUTH_SECRET,
  providers,
  adapter: isE2E ? undefined : SupabaseAdapter({
    url: supabaseUrl,
    secret: supabaseKey,
  }),
})
