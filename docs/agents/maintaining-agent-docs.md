# Maintaining agent instruction files

Project rules and detailed agent guidance live in [`CLAUDE.md`](../../CLAUDE.md) and in this `docs/agents/` directory — not in a monolithic root `AGENTS.md`.

When updating agent instruction files, follow these principles:

- **No hardcoded counts** — Don't write "10 crates" or "5 modules"; these become outdated instantly
- **No exhaustive lists** — Prefer dynamic commands (`ls crates/`) over maintaining complete lists
- **Document constraints, not descriptions** — Focus on non-obvious behaviors, gotchas, and cross-crate dependencies
- **Use nested CLAUDE.md** — Place crate-specific details in `crates/{name}/CLAUDE.md` (Claude Code loads nested `CLAUDE.md` files); keep a short pointer in `crates/{name}/AGENTS.md` when needed
- **Verify before documenting** — Grep/read the code to confirm claims are accurate
- **Delete outdated info** — Outdated docs are worse than no docs
