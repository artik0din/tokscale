# Authoring PR/Issue Content via `gh` CLI

When writing PR bodies, issue bodies, or comments through `gh pr create`, `gh issue comment`, etc.: **prefer `--body-file` with a written-out file over inline `--body` heredocs.**

Why: inline `--body "$(cat <<'EOF' ... EOF)"` patterns lead to recurring mistakes where backticks are written as `` \` `` (incorrectly escaping them inside a single-quoted heredoc where no escaping is needed). The literal `` \` `` then renders in GitHub markdown as backslash + backtick instead of inline code formatting.

**DO:**

- Write the body to `/tmp/pr-body.md` (or similar) with the `Write` tool, then `gh pr create --body-file /tmp/pr-body.md`
- For comment edits: `gh api -X PATCH repos/<owner>/<repo>/issues/comments/<id> -F body=@/tmp/comment.md`

**DON'T:**

- Use `gh pr create --body "$(cat <<'EOF' ... \` ... \` ... EOF)"` — single-quoted heredoc means backticks are already literal; escaping with `` \` `` is wrong and renders incorrectly.
- Use double-quoted heredoc for bodies containing backticks unless you intend command substitution.
- Hard-wrap paragraphs at ~80 columns inside PR/issue/comment bodies. GitHub's markdown collapses single newlines into spaces so the rendered output looks fine, but the **raw markdown view is what reviewers and authors edit in**, and mid-sentence line breaks read as if the prose was chopped. Write each paragraph as one continuous line and let the renderer wrap it. Same rule for blockquotes, list items that span multiple lines, and table cells. Hard wraps are still fine inside fenced code blocks where preserving line layout matters.

This applies to all GitHub-content authoring through the CLI — PR bodies, issue bodies, comments, edits. Commit message bodies should also follow this rule: write prose paragraphs as continuous lines, not hard-wrapped at 80 columns.
