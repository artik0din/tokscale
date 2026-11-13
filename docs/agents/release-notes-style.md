# Release Notes Style

## Title Conventions

| Release Type | Title Format |
|-------------|--------------|
| Standard patch/minor | `` `tokscale@vX.Y.Z` is here! `` |
| Flagship feature | `` EMOJI `tokscale@vX.Y.Z` is here! (Short subtitle with [link](...)) `` |
| Feature spotlight | Custom banner image replacing the standard hero + call-to-action |

**Examples from past releases:**
- Standard: `` `tokscale@v1.1.2` is here! ``
- Flagship: `` 🦞 `tokscale@v1.2.0` is here! (Now supports [OpenClaw](https://github.com/openclaw/openclaw)) ``
- Spotlight: Custom Wrapped 2025 banner + `` Generate your Wrapped 2025 with `tokscale@v1.0.16` ``

## Release Notes Template

```markdown
<div align="center">

[![Tokscale](https://github.com/junhoyeo/tokscale/raw/main/.github/assets/hero-v3.png)](https://github.com/junhoyeo/tokscale)

# `tokscale@vX.Y.Z` is here!
</div>

## What's Changed
* scope(area): description by @author in https://github.com/junhoyeo/tokscale/pull/NNN
* scope(area): description by @author in https://github.com/junhoyeo/tokscale/pull/NNN

## New Contributors
* @username made their first contribution in https://github.com/junhoyeo/tokscale/pull/NNN

**Full Changelog**: https://github.com/junhoyeo/tokscale/compare/vPREVIOUS...vNEW
```

## Style Rules

| Element | Rule |
|---------|------|
| **Header** | Always centered `<div align="center">` with hero banner image linked to the repo |
| **Title** | Backtick-wrapped `tokscale@vX.Y.Z` — package name, not just version |
| **PR list** | `* scope(area): description by @author in URL` — mirrors the PR title exactly as merged |
| **Optional summary** | For releases with many changes or when PR titles alone don't convey impact, add a brief bullet list between the title and "What's Changed" (see v1.0.18 as example) |
| **New Contributors** | Include section when there are first-time contributors |
| **Full Changelog** | Always present at bottom as a GitHub compare link `vPREV...vNEW` |
| **Tone** | Concise. No prose paragraphs. Let the PR list speak for itself. |
| **No draft issues** | Never reference draft release issues (e.g., #121) in the notes |

## When to Add a Summary Block

Add a short bullet list summary (before "What's Changed") when:
- The release has 4+ PRs spanning different areas
- PR titles alone don't convey the user-facing impact
- A new client/integration is the headline

**Example (v1.0.18):**
```markdown
- Improved model price resolver (Rust)
- Add support for Amp (AmpCode) and Droid (Factory Droid)
- Improved sorting feature on TUI
```
