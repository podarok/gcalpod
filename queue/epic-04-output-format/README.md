# Epic 04 — Output format

Cross-cutting `--format <fmt>` support across read commands.
Implementation phases live in W0-P6, W1-P2 (agenda/search), W2-P1
(formatters). This epic tracks the shared infrastructure.

## Scope

- `OutputFormat` enum (`Table | Json | Tsv | Csv | Raw | Conky |
  Template`) in `src/util/format.rs`.
- Trait `Renderable` for command output types.
- Auto-detect tty for json pretty/compact toggle.
- `csv` + `tera` deps (Cargo.toml).
- `--json` legacy flag = alias for `--format json`.

## Done when

- Every read-only command (list, agenda, search, calendars list,
  auth status) supports `--format`.
- Pretty/compact JSON toggle on tty detection.
- `csv` + `tsv` round-trip identical for fixture data.
- `raw` preserves all upstream Event fields.
- Conky + template formats parse fixture without runtime panic.

## Out of scope

- Format negotiation for write commands (add/edit/delete) — they
  print human-friendly success line; no machine output needed.
