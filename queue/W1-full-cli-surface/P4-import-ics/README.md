# W1-P4 — `gcal import <ics-path>`

## Goal

Bulk-insert events from an ICS / VCAL file into a calendar.

## Surface

```
gcal import <path> [--calendar <id>] [--dry-run] [--skip-duplicates]
```

`--dry-run`: parse + validate, print summary, do not insert.
`--skip-duplicates`: skip events whose UID already exists in target.

## Files

- `src/commands/import.rs` (new).
- `Cargo.toml` — add `ical` crate (RFC 5545 parser).

## Tests

- Unit: parse fixture .ics → expected `Vec<Event>`.
- Integration: `--dry-run` performs no API call.

## Validation

```sh
gcal import ~/Downloads/holidays.ics --calendar custom@example.com --dry-run
gcal import event.ics --skip-duplicates
```

## Out of scope

- Export (`gcal export`) — different phase if requested.
- Recurring event reconstruction edge cases — start with simple events.

## Result

_Filled when phase closes._
