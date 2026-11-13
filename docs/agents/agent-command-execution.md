# Agent Command Execution

- When running `tokscale` CLI commands from an automated agent (tests, CI, or tool-driven shells), always pass `--no-spinner` unless spinner behavior is the thing being tested.
- This avoids non-interactive terminal issues and keeps command output stable for assertions and logs.

## Searching the codebase with `rg`

**An empty `rg` result and a malformed `rg` command look identical.** Both print nothing. This is the failure that matters: a broken search reads as "I verified this does not exist," and that conclusion then gets stated as fact. Before concluding something is absent, run a control pattern you *know* matches the same file and confirm it does:

```sh
rg -c "fn parse_opencodereview_file" "$F"   # control: must print 1
rg -c "cost" "$F"                           # only now is 0 meaningful
```

`rg` is not `grep`, and the differences are silent rather than loud:

- **`-r` is `--replace`, not `--recursive`.** `rg -rn 'pattern' path` parses as "replace matches with `n`" and prints plausible-looking transformed output. `rg` recurses by default; there is no `-r` to add.
- **`--include` does not exist.** That is a `grep` flag. Use `-g/--glob` (`rg -g '*.ts' pattern`).
- **The pattern is a regex, so shell/template syntax breaks it.** `${` is a repetition quantifier and `(` opens a group, so searching for `SUM(${submittedDevices.totalActiveTimeMs})` or `for (const day of` is a parse error — or worse, matches the wrong thing. Use `-F` for literal searches:

```sh
rg -nF 'GREATEST(${submittedDevices.totalActiveTimeMs}' "$R"
```

Default to `-F` when the pattern contains any of `$ { } ( ) [ ] | * + ? . \` and you mean them literally — which, in this codebase, is most searches for drizzle `sql` template fragments or Rust macro bodies.
