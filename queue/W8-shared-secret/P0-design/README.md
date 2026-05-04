# W8-P0 — Design

## Goal

Decide UX + storage for shared OAuth secret across profiles.

## Decisions captured

- Shared path: `~/.gcal/secret.json` (re-uses existing legacy
  fallback in resolution chain).
- Sentinel: `~/.gcal/shared.flag` empty file — toggles migration
  behaviour.
- New flag: `gcal init --shared` writes secret to shared path +
  creates sentinel + skips per-profile migration.
- Resolution chain unchanged (env > GCAL_SECRET_FILE > profile
  secret > shared).
- Tokens always per-profile.

## Result

Done. Hand-off to P1.
