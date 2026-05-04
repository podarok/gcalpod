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

Implemented 2026-05-04 on `main`.

Surface added: `gcal quick <text> [--calendar <id>] [--conference]`.
Top-level `gcal "<text>" "[<time>]" [--conference]` retained as
back-compat alias. `add` already supported `--conference`.

Conference on quick-add: `events.quick_add` returns the created
event, then `events.patch(patch_event, cal, id).conference_data_version(1)`
attaches the `hangoutsMeet` ConferenceData via two API calls. Original
quick-add date inference preserved.

Files:
- `src/main.rs` — register `quick` clap subcommand + dispatch arm
  with the patch-after-create flow. Module split deferred to
  `epic-01-cli-restructure`.

`gcal --help` now shows 13 top-level commands. `quick --help` lists
`text`, `--calendar`, `-c/--conference`, inherited `--profile`.
