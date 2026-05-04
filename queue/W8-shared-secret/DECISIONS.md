# W8 Decisions

Append-only.

---

## 2026-05-04 — Use existing legacy fallback as canonical "shared" path

Drop `~/.gcal/shared/secret.json` design — keep `~/.gcal/secret.json`
as the documented shared secret path. Legacy migration still moves
it into `profiles/default/` ONLY when `default` profile is fresh.

For shared-mode users:
- After `gcal init --shared` (new flag) the JSON lands at
  `~/.gcal/secret.json` and migration is suppressed.
- Resolution chain unchanged.

## 2026-05-04 — `gcal init --shared` flag

Adds explicit shared-mode entry. Without `--shared`, init keeps
per-profile behaviour (current).

## 2026-05-04 — Suppress migration when secret is shared

`Profile::migrate_legacy_if_needed()` currently moves
`~/.gcal/{secret,store}.json` to `profiles/default/` on first
default-profile run. Update: if a sentinel file
`~/.gcal/shared.flag` is present, only migrate `store.json` (so
shared `secret.json` stays in place). Created by `init --shared`.

## 2026-05-04 — Token still per profile

Tokens always live at `~/.gcal/profiles/<name>/store.json`. Only
the OAuth client metadata is shareable. (Sharing tokens would
defeat the multi-account purpose.)
