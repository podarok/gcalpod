# W1-P3 — `gcal edit` + `gcal delete`

## Goal

Mutate existing events: edit (interactive editor) + delete (with
confirmation gate).

## Surface

```
gcal edit <event-id> [--calendar <id>] [--field key=value]...
gcal delete <event-id> [--calendar <id>] [--yes]
```

`edit` flow:
- Without `--field`: open `$EDITOR` with current event YAML/JSON.
- With `--field`: in-place mutate without editor (e.g.
  `--field summary="new title" --field start=2026-05-05T15:00`).

`delete` flow:
- Confirmation prompt unless `--yes`.
- Print `summary` + `start` before confirm to avoid mistakes.

## Files

- `src/commands/events/{edit,delete}.rs`.
- `src/util/editor.rs` — spawn `$EDITOR` on tempfile, parse on save.

## Tests

- Edit roundtrip: serialize → edit (no-op) → deserialize → equal.
- Delete confirmation gate: rejects empty / "no" answer.

## Validation

```sh
gcal list --format json | jq -r '.[0].id' | xargs -I{} gcal edit {}
gcal delete <id>           # prompts confirm
gcal delete <id> --yes     # skips
```

## Out of scope

- Recurring event series edit/delete (single-instance only for MVP).
- Bulk delete — out of W1.

## Result

Implemented 2026-05-04 on `main`.

Surface:
- `gcal edit <event-id> [--calendar] [--field key=value]...`
- `gcal delete <event-id> [--calendar] [-y|--yes]`

Files:
- `src/commands/events_mutate.rs` (new) — `EditArgs`, `DeleteArgs`,
  `edit(hub, args)`, `delete(hub, args)`. Edit fetches the event,
  applies `--field` mutations (supported keys: `summary`,
  `description`, `location`, `start`, `end`), then calls
  `events.update`. Delete prints `id`/`summary`/`start` for
  confirmation, prompts unless `--yes`, then calls `events.delete`.
- `src/commands/mod.rs` — register `events_mutate`.
- `src/main.rs` — register `edit` + `delete` clap subcommands +
  dispatch (both need hub).

`start` / `end` use `parse_range_input` (same parser as `list
--from/--to`), so natural inputs like `+7d`, `tomorrow`, RFC3339
all work.

Editor mode (open `$EDITOR` with full event JSON) deferred — the
`--field key=value` form covers the common tweak case and avoids
the `tempfile` + editor-spawn complexity. Add later if needed.

Recurring-series edit/delete remain out of MVP per plan.

Smoke: `gcal edit --help` + `gcal delete --help` show all flags.
Build clean.
