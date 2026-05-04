# W2-P0 — `gcal remind`

## Goal

Run a shell command N minutes before the next event starts.
Replicates `gcalcli remind`.

## Surface

```
gcal remind <mins> -- <command>...
```

Example:
```
gcal remind 5 -- terminal-notifier -title "Meeting" -message "{{summary}}"
```

`{{summary}}`, `{{start}}`, `{{html_link}}` interpolated into the
command before exec.

## Files

- `src/commands/remind.rs` (new).
- `src/util/template.rs` (new) — small `{{var}}` substitution.

## Tests

- Unit: template substitution + missing-var handling.
- Unit: exec arg quoting (no shell injection).

## Validation

Manual — run with imminent event, verify command fires once.

## Out of scope

- Daemon mode (`remind --watch` running in background) — not needed
  if user pairs with `cron` / `launchd`.

## Result

Implemented 2026-05-04 on `main`.

Surface: `gcal remind <mins> [--calendar <id>] -- <command> [args...]`.

Files:
- `src/commands/remind.rs` (new) — `RemindArgs`, `run(hub, args)`.
  Pulls events in `[now, now + mins]`, sorts by start, picks first
  with a date-time start. Interpolates `{{summary}}`, `{{start}}`
  (RFC3339 in user TZ), `{{html_link}}` into every command token,
  then exec via `std::process::Command`.
- `src/commands/mod.rs` — register `remind`.
- `src/main.rs` — register `remind` clap subcommand with
  `trailing_var_arg(true)` so `-- terminal-notifier ...` style
  works. Dispatch uses hub.

If no event in window: prints "no events in the next N minute(s)".
If command exits non-zero: stderr line includes status + invocation.

Skipped separate `src/util/template.rs` module — substitution is a
3-line `String::replace` chain done inline. Add module if substitution
grows beyond the three vars.
