# W2-P1 — Formatters

## Goal

Add `conky` + `template` (Tera) modes to `--format`.

## Surface

```
gcal list --format conky
gcal list --format template --tpl ~/.gcal/templates/agenda.tera
```

## Files

- `src/util/format.rs` — extend `OutputFormat` enum.
- `Cargo.toml` — add `tera` (or `minijinja` for smaller footprint).

## Tests

- Snapshot: conky output for fixture event set.
- Snapshot: simple Tera template render.

## Validation

Conky integration: drop `${execi 60 gcal list --format conky}` into
`~/.conkyrc` → renders.

## Out of scope

- Markdown / RTF formats — add only on user request.

## Result

Implemented 2026-05-04 on `main`. Conky variant only — Tera template
deferred (heavy dep, low priority for v1).

Files:
- `src/util/format.rs` — `OutputFormat::Conky` variant +
  `render_conky()`: one line per event,
  `${color cyan}YYYY-MM-DD HH:MM${color} <summary>`.
- `src/main.rs` — list/agenda/search format value_parser includes
  `conky`. calendars list rejects with hint.
