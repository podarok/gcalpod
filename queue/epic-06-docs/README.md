# Epic 06 — Documentation

README + `docs/custom_auth.md` + man page generation. Cross-cuts every
wave (each user-visible change updates docs).

## Scope

- `README.md` — top-level: install, quickstart, command summary
  table linking deeper into `docs/`.
- `docs/custom_auth.md` — Google Console walk-through (already
  updated in W0-P0).
- `docs/profiles.md` — multi-profile workflow + storage layout.
- `docs/output-formats.md` — table / json / tsv / csv / raw / conky /
  template examples.
- `docs/troubleshooting.md` — common errors + fixes.
- `man gcal` (W2-P2) — generated from clap.

## Done when

- README has "Quickstart" that gets a fresh user to first event in
  <5 min.
- Every `--flag` documented in clap help (visible via
  `gcal <subcommand> --help`).
- `docs/profiles.md` covers add / switch / remove flow.

## Out of scope

- Asciinema recordings — nice-to-have, not required.
- Translated docs (UA/EN only English for now).
