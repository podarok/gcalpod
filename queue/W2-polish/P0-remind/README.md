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

_Filled when phase closes._
