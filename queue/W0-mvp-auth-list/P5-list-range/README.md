# P5 — `gcal list --from --to`

## Goal

Replace fixed "current week" window with explicit date range.
Critical for stm32 sprint planning use case (next 30/60 days).

## Surface

```
gcal list [--from <date>] [--to <date>] [--calendar <id>] [--profile <name>]
```

Flags:
- `--from <date>` — RFC3339 or natural ("today", "monday",
  "2026-05-04"). Default: start of current week.
- `--to <date>` — RFC3339 or natural ("+30d", "next month").
  Default: `--from + 7d`.
- `--calendar <id>` — calendar ID or `primary` (default).
- `--profile <name>` — global flag, applies everywhere.

## Files

- `src/commands/events/list.rs` — extract from `main.rs`.
- `src/util/date.rs` — extend `get_date_from_string` to support
  relative dates (`+30d`, `next monday`).
- `src/main.rs` — register `list` under `events` subcommand group
  AND keep top-level alias for back-compat.

## Steps

1. Move event-list logic from `main.rs` into
   `commands/events/list.rs`. Keep current week as default.
2. Add `--from` / `--to` parsing. Reuse `chrono::DateTime::parse_from_rfc3339`
   for RFC3339; add small parser for `+Nd`, `+Nw`, "today",
   "tomorrow", weekday names.
3. Pass parsed `time_min` / `time_max` to `events.list()`.
4. Add `--calendar` flag → forward to `events.list("<id>")`.
5. Keep table renderer; switch to flat list when range > 14 days
   (table becomes unreadable).

## Tests

- Unit: relative-date parser (`+7d`, `next monday`, `today`).
- Unit: range > 14d → flat list renderer used.
- Integration: mocked `CalendarHub` returns fixture events,
  verify `time_min`/`time_max` passed through.

## Validation

```sh
gcal list                              # current week (back-compat)
gcal list --from today --to +7d        # natural
gcal list --from 2026-05-01 --to 2026-05-31
gcal list --calendar work@example.com
```

## Out of scope

- JSON output (P6).
- Search query (W1-P2).
- Multi-calendar merge (W1 enhancement).

## Result

_Filled when phase closes._
