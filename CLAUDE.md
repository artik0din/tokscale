# Tokscale

CLI and dashboard for tracking token usage and costs across AI coding agents (see README).

## Stack

- Rust workspace (toolchain in `rust-toolchain.toml`): `crates/tokscale-core`, `crates/tokscale-cli` (binary `tokscale`)
- Bun/npm workspaces: `packages/cli`, `packages/tokscale`, `packages/cli-*`, `packages/frontend`, `packages/benchmarks`
- Web: Next.js + Drizzle in `packages/frontend`

## Layout

- `crates/tokscale-core/` — parsers, pricing, shared logic
- `crates/tokscale-cli/` — CLI/TUI entrypoint
- `packages/cli/` — npm dispatcher (`@tokscale/cli`) and platform optionalDependencies
- `packages/frontend/` — Next.js app, Drizzle migrations in `src/lib/db/`
- `.github/workflows/` — CI; release via `publish-cli.yml`

## Rules

- Conventional Commits for commits and PR titles; no internal review labels (P0, Oracle, hardening, etc.) in titles or commits.
- Before any commit: inspect `git config user.name` / `user.email` and remotes; stop and ask if identity does not match the expected contributor/automation for the branch. Never commit as `worker*`, `*@example.invalid`, etc. Merge with `gh pr merge --merge` unless asked otherwise; verify merge commit title before merging. Read `docs/agents/git-and-commits.md` before committing or merging.
- Automated `tokscale` runs: pass `--no-spinner` unless spinner behavior is under test.
- Drizzle migrations: do not hand-edit `packages/frontend/src/lib/db/migrations/meta/_journal.json` or SQL after apply — see `docs/agents/migrations.md`.
- `rg`: empty output can mean a broken command — confirm with a control pattern. Use `-g` not `--include`; `-F` when the pattern has `$ { } ( ) [ ] | * + ? . \`` literally.

## Install / test / lint / build / run

From repo root (Rust: `rust-toolchain.toml`; Bun: CI pins 1.3.10 frontend/launcher, 1.1.38 publish/release — not `bun.lock`):

```sh
bun install --frozen-lockfile
cargo test --workspace --all-features --no-fail-fast
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-features -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps --workspace
bun run --cwd packages/frontend test
bun run --cwd packages/frontend typecheck
bun run test:launchers
python3 scripts/check-workflow-toolchain-paths.py
cargo build --release -p tokscale-cli
bun run build
bun run cli -- --no-spinner --help
```

CI-only: macOS `tui_startup` (`test_coverage.yml`); release scripts (`release_validation.yml`). More: `docs/agents/`.

## Detailed docs

- `docs/agents/maintaining-agent-docs.md` — editing agent files
- `docs/agents/gh-cli-authoring.md` — `gh` PR/issue bodies
- `docs/agents/git-and-commits.md` — identity, merges, commit/PR titles
- `docs/agents/migrations.md` — Drizzle journal and applied SQL
- `docs/agents/agent-command-execution.md` — `--no-spinner`, `rg` pitfalls
- `docs/agents/release-and-deployment.md` — publish workflow, checklist
- `docs/agents/release-notes-style.md` — GitHub Release layout
- `docs/agents/history.md` — migration journal one-off exception
