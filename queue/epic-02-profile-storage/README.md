# Epic 02 — Profile storage

Multi-profile credential + token storage at
`~/.gcal/profiles/<name>/`. Legacy `~/.gcal/secret.json` /
`~/.gcal/store.json` still load as `default` profile.

## Scope

Implementation lives in `W0-mvp-auth-list/P1-profiles-storage/`.
This epic tracks **all** ramifications across the codebase.

- `Profile` + `Config` types in `src/profile.rs`, `src/config.rs`.
- `ApplicationSecret` resolution chain expanded to consult profile dir.
- `auth()` token persistence path becomes profile-relative.
- All subcommands accept global `--profile <name>` flag.
- Migration: legacy → `profiles/default/` happens once, idempotent.

## Done when

- A user with only legacy `~/.gcal/secret.json` runs any command and
  the layout migrates automatically with one stderr line.
- `gcal auth login --profile work` creates fresh `profiles/work/`.
- Switching profiles works mid-session via `--profile` flag.
- Env vars override profile files (preserves W0-P0 contract).

## Out of scope

- Auth subcommand surface (epic-01 + W0 phases).
- Token revocation (W0-P4).
