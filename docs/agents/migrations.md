# Migration journal hygiene

Migrations 0010 and 0011 are a one-time journal timestamp exception — see [`history.md`](history.md).

Never hand-edit `packages/frontend/src/lib/db/migrations/meta/_journal.json` timestamps or sequence numbers. Always run `drizzle-kit generate` to claim a migration slot — the tool assigns the correct monotonic index and timestamp atomically.

If two branches generate migrations with the same index, resolve the conflict by re-running `drizzle-kit generate` on the branch that was merged later — do not manually renumber files or edit `_journal.json`.

**Never edit the SQL of a migration file after it has been applied to any database.** drizzle stores the SHA256 of the migration content in `drizzle.__drizzle_migrations` on first apply. If the local file content changes (even just a comment), the local hash diverges from the stored hash and drizzle-kit migrate will treat the migration as missing and attempt to re-apply it — which fails on idempotent-unsafe DDL. If you need to document a migration after the fact (lock-window risk, rollback notes, anything), put the commentary in a sidecar `0NNN_*.md` next to the .sql, in `schema.ts`, or in this doc — never as comments inside the applied .sql.
