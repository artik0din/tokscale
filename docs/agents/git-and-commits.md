# Git Identity & Merge Discipline

- Before any commit, inspect the effective Git identity (`git config user.name` / `user.email`) and remotes. If the identity does not match the contributor or expected automation account for the current branch, stop and ask for confirmation.
- Never commit as worker/agent identities such as `worker1`, `worker2`, `worker3`, or `*@example.invalid`.
- When merging pull requests through `gh`, use merge commits (`gh pr merge --merge ...`) unless the user explicitly requests another merge strategy.
- Before merging, verify the merge commit title is the intended conventional PR title and does not contain worker/agent/internal review jargon.

## Commit Message Convention

```
<type>: <description>

[optional body]
```

### Types

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `refactor` | Code refactoring (no behavior change) |
| `docs` | Documentation only |
| `test` | Adding or updating tests |
| `chore` | Maintenance tasks |
| `perf` | Performance improvements |

### Examples

```
feat: add session branching with /fork command
fix: handle empty response from provider
refactor: extract streaming logic to separate module
docs: update README with new CLI options
```

### Commit Message & PR Title Rules (CRITICAL)

> These rules apply to **both commit messages AND pull request titles**. PR titles become the squash-merge commit message, so they must follow the same conventions.

**DO:**
- Describe the actual change in plain, technical terms
- Keep commits atomic (one logical change per commit)
- Use the format: `<type>(<scope>): <what changed and why>`

**DON'T:**
- Reference internal review labels (P0, P1, P2, etc.) in commits or PR titles
- Mention "Oracle", "audit", "review findings", "hardening" in commits or PR titles
- Use agent-internal jargon: "wave", "hardening", "compliance", "verification pass"
- Bundle multiple unrelated fixes into one commit
- Use vague messages like "fix issues" or "address feedback"

**Good Examples:**
```
fix(lsp): pass server args to stdio spawn command
fix(lsp): convert 1-indexed input lines to 0-indexed LSP positions
fix(gemini): parse SSE data frames instead of raw JSON lines
fix(orchestrator): route provider tools through approval flow
```

**Bad Examples (NEVER do this):**
```
fix: address P0 issues from Oracle review      ❌
fix(hardening): Oracle Round 4 fixes           ❌
fix: audit findings                            ❌
fix: various improvements                      ❌
fix(tui): harden unreleased changes — P0-P3    ❌  (PR title)
fix: hardening wave 1 compliance fixes         ❌  (PR title)
```
