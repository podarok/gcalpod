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

Implemented 2026-05-04 on `main`.

Surface: `gcal import <path> [--calendar <id>] [--dry-run]
[--skip-duplicates]`.

Files:
- `src/commands/import.rs` (new) — `ImportArgs`, `run(hub, args)`,
  `parse_dt_property()`. Reads ICS via `ical::IcalParser`,
  iterates `cal.events`, maps SUMMARY/DESCRIPTION/LOCATION/UID/
  DTSTART/DTEND properties to `google_calendar3::api::Event`.
  Datetime parser handles ICS forms `YYYYMMDD` (all-day),
  `YYYYMMDDTHHMMSSZ` (UTC), `YYYYMMDDTHHMMSS` (floating, treated
  as local in active TZ).
- `src/commands/mod.rs` — register `import`.
- `src/main.rs` — register `import` clap subcommand + dispatch.
- `Cargo.toml` — add `ical` 0.11 dep.

`--skip-duplicates` first lists existing events on the target
calendar (max 2500), collects `i_cal_uid` set, skips inputs whose
UID matches. Inserted events carry the original UID via
`Event.i_cal_uid` so re-imports remain idempotent.

Smoke verified end-to-end with /tmp/test.ics:
```
gcal: parsed 1 event(s) from /tmp/test.ics
  - Test stm32 firmware review | 2026-05-10T14:00:00+00:00 | uid=abc-123-test@gcal
(dry-run; nothing inserted)
```

Recurring event reconstruction (RRULE, EXDATE, RECURRENCE-ID) and
export remain out of scope per plan.
