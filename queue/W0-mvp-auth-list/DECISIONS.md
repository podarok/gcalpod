# W0 Decisions

Append-only. Newest first.

---

## 2026-05-04 — Profile storage layout

- `~/.gcal/profiles/<name>/secret.json` + `store.json` per profile.
- `~/.gcal/config.toml` holds `active_profile`, `default_calendar`,
  `tz`. Created lazily on first `auth login`.
- Legacy `~/.gcal/secret.json` and `~/.gcal/store.json` still read
  as `default` profile if the new dir layout is empty.
- Migration: first `gcal auth login --profile default` moves legacy
  files into `~/.gcal/profiles/default/` (move, not copy). Print one
  stderr line announcing the move.

## 2026-05-04 — Token expiry display

- `gcal auth status` reads `store.json` directly, parses
  `token.expires_in` + `token.fetched_at` to compute remaining
  validity. No live API call required for happy path.
- If token missing → status reports "not authenticated".
- Live API ping (`hub.calendars().get("primary")`) gated behind
  `--check` flag (default off) to keep `status` fast.

## 2026-05-04 — Backwards compat with current `feat/custom-oauth-config`

- Env vars `GCAL_CLIENT_ID` / `GCAL_CLIENT_SECRET` /
  `GCAL_PROJECT_ID` / `GCAL_SECRET_FILE` / `GCAL_VERBOSE`
  continue to resolve regardless of profile. They override profile
  files (env > profile > legacy > fallback).
- `--profile <name>` flag wins over `GCAL_PROFILE` env var, which
  wins over `config.toml` `active_profile`.

## 2026-05-04 — Output format convention

- `--format <fmt>` flag on `list`: `table` (default) | `json` | `tsv`
  | `csv` | `raw`. `--json` kept as ergonomic alias for `--format json`.
- Owner request 2026-05-04: planning workflow needs TSV/CSV (spreadsheet)
  + JSON (jq) + raw (escape hatch for fields not in v1 schema).
- v1 stable schema: `{id, calendar_id, summary, start, end, all_day,
  status, creator, attendees[], html_link}`. Semver: bump on
  rename/removal.
- TSV header row first; columns identical between TSV and CSV.
- Pretty JSON only when stdout is tty; compact when piped (saves
  tokens for `jq`).
- `raw` format pretty-prints upstream
  `google_calendar3::api::Event` JSON unchanged — escape hatch for
  recurrence rules, extended properties, attachments, etc.
- Future: per-command `--format` (not just `list`) — defer to W1.
