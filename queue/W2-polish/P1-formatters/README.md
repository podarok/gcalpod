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

_Filled when phase closes._
