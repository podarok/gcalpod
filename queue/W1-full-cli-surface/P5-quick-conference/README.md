# W1-P5 — `gcal quick` + conference flag polish

## Goal

Promote existing top-level "natural language" event creation to an
explicit `quick` subcommand. Ensure `--conference` works on both
`add` and `quick`.

## Surface

```
gcal quick "<natural language>" [--calendar <id>] [--conference]
gcal add <title> <date> [--calendar <id>] [--conference]
```

Top-level `gcal "..."` retained as alias for `quick`.

## Files

- `src/commands/events/{add,quick}.rs` — extracted from `main.rs`.

## Tests

- Snapshot: both subcommands produce same Event shape for equivalent
  input.
- `--conference` adds `ConferenceData` exactly once (regression
  test for upstream UUID generation bug if any).

## Validation

```sh
gcal quick "Retro & Demo at 16:00" --conference
gcal add "Sprint planning" "2026-05-06 10:00" --conference
gcal "Lunch" "13:00"     # alias still works
```

## Out of scope

- Custom conference solutions (only Hangouts Meet supported by API).

## Result

_Filled when phase closes._
