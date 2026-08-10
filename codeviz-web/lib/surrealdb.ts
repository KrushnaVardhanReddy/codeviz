import { Surreal } from "surrealdb";

const db = new Surreal();

export const clientPromise: Promise<Surreal> = (async () => {
  if (process.env.npm_lifecycle_event === 'build') {
    return db;
  }

  const url    = process.env.SURREALDB_URL  ?? "http://127.0.0.1:8000/rpc";
  const user   = process.env.SURREALDB_USER ?? "root";
  const pass   = process.env.SURREALDB_PASS ?? "root";
  const ns     = process.env.SURREALDB_NS   ?? "codeviz";
  const dbName = process.env.SURREALDB_DB   ?? "main";

  await db.connect(url, {
    namespace: ns,
    database: dbName,
  });
  await db.signin({
    username: user,
    password: pass,
  });
  return db;
})();
