# Historical notes

Migrations 0010 and 0011 have round-number hand-edited timestamps (`"when": 1780000000000` and `"when": 1780086400000`) as a one-time historical exception made during the 2026-05-25 schema audit. No future migration should follow this pattern; use `drizzle-kit generate` exclusively.
