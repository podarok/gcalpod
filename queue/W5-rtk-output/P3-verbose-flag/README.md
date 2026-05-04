# W5-P3 — `-v/--verbose` flag

## Goal

Add a learner-friendly verbose mode. Owner request 2026-05-04:
default output stays compact (RTK style); `--verbose` expands with
context, hints, examples — useful for newcomers and AI init agents
bootstrapping a new gcal install.

## Surface

```
gcal --verbose <subcommand>...
gcal -v <subcommand>...
```

Global flag, propagates to every command.

## Behaviour by command

- `auth login --verbose` — prints OAuth scope explanations, profile
  path, what the browser will ask, what to do if browser doesn't open.
- `auth status --verbose` — adds "what each field means" annotations
  + how to fix `not authenticated`.
- `list --verbose` — adds a 2-line preamble: range used, calendar id,
  format chosen, secret source. Below renders.
- `config set --verbose` — echoes resolved config.toml path + reason
  for any validation success.
- `init --verbose` — prints why each Google Console step matters.
- Errors: when `--verbose`, full body printed inline (no log-tee
  needed for short errors).

## Files

- `src/main.rs` — global `-v/--verbose` flag (clap).
- Per-command modules — branch on `verbose: bool` arg.
- `src/util/recovery.rs` — when verbose, print full error body
  inline instead of summary + log path.

## Tests

- Snapshot: `gcal auth status --verbose` (vs default) byte length
  increases by ≥ 50%.
- Snapshot: `gcal list --verbose` includes preamble line.
- Recovery: error message identical text in verbose log.

## Validation

```sh
gcal list --verbose
gcal auth login --profile work -v
gcal init --verbose
```

## Out of scope

- Translating verbose text (English only).
- TUI tooltips.

## Result

_Filled when shipped._
