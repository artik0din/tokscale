# Release & Deployment

## Overview

Releases are published to npm via a GitHub Actions `workflow_dispatch` pipeline. There is no staging environment — publishes go directly to npm `latest`.

## Release Pipeline

**Workflow:** `.github/workflows/publish-cli.yml`

**Trigger:** Manual — GitHub Actions UI → "Publish" → "Run workflow"

**Inputs:**
- `bump`: Version bump type — `patch (x.x.X)` | `minor (x.X.0)` | `major (X.0.0)`
- `version` (optional): Override string (e.g., `2.0.0-beta.1`), takes precedence over bump
- `recovery` (optional): Retry an already committed release version. Requires `version` and reuses the current release commit when the manifests already match.

**Stages (sequential):**

| Job | Description |
|-----|-------------|
| `bump-versions` | Reads current version from `packages/cli/package.json`, calculates new version, updates the Rust workspace version plus `Cargo.lock`, the CLI, wrapper, and platform package manifests, then uploads the bumped release files as an artifact |
| `build-cli-binary` | Builds the native Rust binaries defined by the workflow matrix |
| `prepare-release-provenance` | Checks npm auth/release state, then commits and pushes the release provenance files as `chore: bump version to X.Y.Z`. In recovery mode, reuses the already committed release SHA when there are no manifest diffs. |
| `publish-platform-packages` | Publishes platform-specific packages (`@tokscale/cli-darwin-arm64`, etc.) containing native binaries to npm, skipping package versions that already exist only during recovery |
| `publish-cli` | Publishes `@tokscale/cli` to npm (binary dispatcher + optionalDependencies) |
| `publish-alias` | Publishes `tokscale` wrapper package to npm |
| `finalize` | Creates or updates tag `vX.Y.Z` and the GitHub Release after npm publishing succeeds |

**Duration:** ~15-20 minutes end-to-end.

**Package publish chain:** `@tokscale/cli` (with platform packages as optionalDependencies) → `tokscale` (depends on cli). Each waits for the previous to succeed.

## Post-Pipeline: Git Tag & GitHub Release

The CI pipeline creates or updates the git tag and GitHub Release after npm publishing succeeds. After the workflow completes successfully:

1. Verify the `chore: bump version to X.Y.Z` commit was pushed by CI or reused by recovery
2. Verify tag `vX.Y.Z` targets the release provenance commit
3. Verify the GitHub Release exists and follows `docs/agents/release-notes-style.md`

## Versioning Conventions

| Bump Type | When to Use | Example |
|-----------|-------------|---------|
| `patch` | Bug fixes, small features, additive parser support | `1.2.0` → `1.2.1` |
| `minor` | New client support, significant features, UI overhauls | `1.1.2` → `1.2.0` |
| `major` | Breaking changes (never used so far) | `1.2.1` → `2.0.0` |

Release version is stored in the Rust workspace and the npm package manifests, and CI updates them together:
- `Cargo.toml` (`[workspace.package].version`) — Rust binary and exported metadata version
- `Cargo.lock` — local workspace package versions for `tokscale-cli` and `tokscale-core`
- `packages/cli/package.json` — CLI package version and platform optional dependency versions
- Platform packages (`packages/cli-*/package.json`) — native package versions
- `packages/tokscale/package.json` — wrapper version plus `@tokscale/cli` dependency version

## CI-Only Workflow

**`.github/workflows/build-native.yml`** — Runs on PRs that affect the Rust workspace, CLI platform manifests, or native build workflow. Builds the configured native target matrix to verify compilation. Does not publish.

## Deployment Checklist

Never dispatch `publish-cli.yml` (npm `latest`) without an explicit request from Kevin.

```
1. [ ] All target PRs merged to main
2. [ ] `cargo test` passes in crates/tokscale-cli
3. [ ] No open blocker bugs (regressions from changes being released)
4. [ ] Run "Publish" workflow via GitHub Actions UI
   - Select bump type (patch/minor/major)
   - For a failed publish retry, set `version` to the already committed release version and enable `recovery`
   - Wait for all stages to complete
5. [ ] Verify `chore: bump version to X.Y.Z` commit was pushed
6. [ ] Verify packages on npm: @tokscale/cli, tokscale
7. [ ] Verify GitHub Release
   - Tag: vX.Y.Z targeting the bump commit
   - Release notes follow `docs/agents/release-notes-style.md`
8. [ ] Smoke test: `bunx tokscale@latest --version`
```
