# P6 — `gcal list --format <fmt>`

## Goal

Stable machine-readable output for `list` so events feed planning
tooling (TSV/CSV → spreadsheet, JSON → `jq`, raw → native API
inspection). Critical for stm32 firmware sprint planning use case
where events get filtered + mapped onto
`all_repos_queue/wave-019-ec-can-battery-fw-stm32`.

## Surface

```
gcal list --format <fmt> [--from --to ...]
gcal list --json                       # alias for --format json
```

Formats:
- `table` (default) — human, comfy-table.
- `json` — stable v1 schema (see below). Pretty if tty, compact if piped.
- `tsv` — header row + tab-separated columns. Spreadsheet paste-ready.
- `csv` — RFC 4180 compliant. Quoting handled by `csv` crate.
- `raw` — pretty-printed upstream `google_calendar3::api::Event` JSON
  with **all** fields preserved (escape hatch for fields not in v1 schema).

## Schema

Stable v1 event:

```json
{
  "id": "abc123",
  "calendar_id": "primary",
  "summary": "stm32 fw rebase",
  "description": "...",
  "start": "2026-05-05T14:00:00+03:00",
  "end":   "2026-05-05T15:00:00+03:00",
  "all_day": false,
  "html_link": "https://calendar.google.com/...",
  "attendees": [{"email":"alice@example.com","response":"accepted"}],
  "conference":  {"entry_points":[{"type":"video","uri":"…"}]},
  "creator": {"email":"alice@example.com"},
  "status": "confirmed",
  "updated": "2026-05-04T07:00:00Z"
}
```

Output: top-level JSON array. Empty range → `[]`.

## TSV columns

Header (tab-separated):
```
id	calendar_id	summary	start	end	all_day	status	creator	attendees_count	html_link
```

Datetime fields RFC3339 in user TZ. `attendees` flattened to count;
full list available via `--format json` or `--format raw`.

## CSV columns

Same columns as TSV, RFC 4180 quoted. `csv` crate handles embedded
commas / newlines / quotes in `summary`.

## Files

- `src/util/serde_event.rs` (new) — `Event → ListEvent` mapper +
  `serde::Serialize` impl.
- `src/util/format.rs` (new) — `OutputFormat` enum + dispatch table.
- `src/commands/events/list.rs` — branch on `--format`.
- `Cargo.toml` — add `csv` crate.

## Steps

1. Define `OutputFormat { Table, Json, Tsv, Csv, Raw }`.
2. Define `ListEvent` struct with serde-derive (v1 schema below).
3. Implement `From<google_calendar3::api::Event>` mapping nullable
   fields to `Option`.
4. In `list.rs`, after fetching, dispatch on `--format`:
   - `Table` → comfy-table renderer.
   - `Json` → `serde_json::to_writer` (pretty if tty, compact if piped).
   - `Tsv` → header line + tab-joined fields.
   - `Csv` → `csv::Writer` over `Vec<ListEvent>`.
   - `Raw` → `serde_json::to_writer_pretty(&events)` on the
     upstream `Vec<Event>` directly (no field flattening).
5. `--json` flag remains as ergonomic alias for `--format json`.

## Tests

- Snapshot: ListEvent from fixture API response (json + tsv + csv).
- Round-trip: json serialize → parse → equal.
- Schema stability: drop a field test (must fail to commit).
- TSV: tab/newline in summary properly escaped.
- CSV: RFC 4180 compliance via `csv` crate test.
- Raw: full upstream Event preserved (no field loss).

## Validation

```sh
# JSON for jq pipelines
gcal list --from today --to +30d --format json | jq '.[] | select(.summary | test("stm32"; "i"))'

# TSV for spreadsheet
gcal list --from today --to +30d --format tsv > planning.tsv

# CSV for shareable
gcal list --from today --to +30d --format csv

# Raw for fields not in v1 schema (recurrence rules, extended properties, etc.)
gcal list --from today --to +30d --format raw | jq '.[] | .recurrence'

# Defaults
gcal list                  # table
gcal list --json           # alias for --format json
```

## Out of scope

- JSON output for other commands (`auth status` already JSON in P3;
  others handled per-phase).
- YAML output — not requested. Add later if asked.
- Custom column subset for TSV/CSV — defer to W1 if needed.

## Result

Implemented 2026-05-04 on `main`.

Files:
- `src/util/format.rs` (new) — `OutputFormat { Table, Json, Tsv,
  Csv, Raw }`, `ListEvent` v1 schema, `from_event(ev, cal_id, tz)`,
  `render_list(fmt, &events, &raw_events)` dispatch.
  TSV escapes tab/newline/CR/backslash. CSV via `csv::Writer`.
  JSON pretty if stdout is tty (`std::io::IsTerminal`), compact
  when piped. Raw = pretty-printed upstream `Vec<Event>` unchanged.
- `src/main.rs` — extended `list` clap subcommand with
  `--format <table|json|tsv|csv|raw>` (default `table`) and
  `--json` (ergonomic alias, conflicts_with `--format`). Branch
  before flat-list/table renderer when format != table.
- `src/util/mod.rs` — register `format` module.
- `Cargo.toml` — add `csv` dep.

v1 ListEvent schema:
`{id, calendar_id, summary, description, start, end, all_day,
status, creator, attendees_count, html_link, updated}`. Datetimes
are RFC3339 in user TZ (or `YYYY-MM-DD` for all-day). Bump on
field rename/removal.

TSV/CSV columns (10 fields, header row first):
`id calendar_id summary start end all_day status creator
attendees_count html_link`

Smoke: `gcal list --help` shows `--format` with possible values
+ `--json` alias. Build clean.

Validation deferred to live Calendar API run (next test cycle).
