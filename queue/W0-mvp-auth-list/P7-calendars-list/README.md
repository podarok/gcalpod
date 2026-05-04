# P7 — `gcal calendars list`

## Goal

List all calendars accessible to the active profile so the user can
pick a non-primary calendar ID for `--calendar` flag usage.

## Surface

```
gcal calendars list [--profile <name>] [--json]
```

## Output (human)

```
ID                          | Summary             | Access | Primary
─────────────────────────────┼─────────────────────┼────────┼─────────
primary                     | alice@example.com   | owner  | ✓
team-firmware@example.com   | Team firmware       | writer |
holidays@google.com         | Ukrainian Holidays  | reader |
```

## Output (JSON)

```json
[
  {"id":"primary","summary":"alice@example.com","access_role":"owner","primary":true,"timezone":"Europe/Kyiv"},
  {"id":"team-firmware@example.com","summary":"Team firmware","access_role":"writer","primary":false}
]
```

## Files

- `src/commands/calendars/list.rs` (new).
- `src/commands/calendars/mod.rs` (new) — group.
- `src/main.rs` — register `calendars` subcommand.

## Steps

1. Call `hub.calendar_list().list().doit().await`.
2. Map to `CalendarSummary { id, summary, access_role, primary,
   timezone }`.
3. Render table (comfy-table) or JSON depending on `--json`.

## Tests

- Snapshot: CLI help output.
- Snapshot: JSON shape from fixture.

## Validation

```sh
gcal calendars list
gcal calendars list --json | jq '.[] | select(.primary)'
```

## Out of scope

- `calendars create` / `delete` / `share` — out of MVP.
- Calendar event CRUD lives under `events` (W1-P3).

## Result

Implemented 2026-05-04 on `main`.

Surface: `gcal calendars list [--format <table|json|tsv|csv|raw>]
[--json]`. Reuses `OutputFormat` from `util::format` (W0-P6).

Files:
- `src/commands/calendars/mod.rs` (new) — group.
- `src/commands/calendars/list.rs` (new) — `ListArgs { format }`,
  `CalendarSummary`, `run(hub, args)`. Renderers for json (tty
  pretty/compact toggle), tsv (tab-escaping), csv (csv crate), raw
  (upstream `Vec<CalendarListEntry>`), table (comfy-table with
  primary `✓` marker).
- `src/commands/mod.rs` — register `calendars`.
- `src/main.rs` — register `calendars list` clap subcommand,
  dispatch after `calendar::auth(&prof)` (needs hub).

TSV/CSV columns: `id summary access_role primary timezone`. JSON
schema:
`{id, summary, description, access_role, primary, timezone, selected}`
(7 fields).

Smoke: `gcal calendars --help` + `gcal calendars list --help` show
correct flags. Build clean.
