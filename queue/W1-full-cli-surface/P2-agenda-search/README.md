# W1-P2 — `gcal agenda` + `gcal search`

## Goal

Two read-only commands beyond `list`:
- `agenda` — flat chronological list (no week grid).
- `search` — full-text query against the Calendar API.

Both honor `--format` from W0-P6.

## Surface

```
gcal agenda [--from <date>] [--to <date>] [--calendar <id>] [--format <fmt>]
gcal search <query> [--from <date>] [--to <date>] [--calendar <id>] [--format <fmt>]
```

`search` uses `events.list().q(query)`.

## Files

- `src/commands/events/{agenda,search}.rs`.
- Reuse `util/format.rs` from W0-P6.

## Tests

- Snapshot: agenda + search help.
- Integration: search forwards `q=` param.

## Validation

```sh
gcal agenda --from today --to +14d --format tsv
gcal search "stm32" --from -90d --to today
gcal search "fw" --calendar team-firmware@example.com --format json
```

## Out of scope

- Mutating commands (W1-P3).

## Result

_Filled when phase closes._
