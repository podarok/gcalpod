# P3 — `gcal auth status`

## Goal

Show current authentication state per profile. Mirrors `gh auth
status`. Default reads token store offline (fast, no API call); add
`--check` for live API ping.

## Surface

```
gcal auth status [--profile <name>] [--all] [--check] [--show-token]
```

Flags:
- `--profile <name>` — single profile (default: active).
- `--all` — show every profile under `~/.gcal/profiles/`.
- `--check` — ping `hub.calendars().get("primary")` to verify token.
- `--show-token` — print bearer (security warning printed).

## Output (human)

```
gcal auth status

  Profile: work (active)
    Account: alice@example.com
    Source:  ~/.gcal/profiles/work/secret.json
    Scopes:  calendar, calendar.events
    Token:   valid for 0h47m (expires 2026-05-04 12:33 UTC)
    State:   ✓ ready

  Profile: personal
    Account: -
    Source:  env (GCAL_CLIENT_ID)
    Scopes:  -
    Token:   not authenticated
    State:   ✗ run `gcal auth login --profile personal`
```

## Output (JSON)

```json
[
  {"profile":"work","active":true,"account":"alice@example.com",
   "source":"profile_file","scopes":["calendar","calendar.events"],
   "token_expires_at":"2026-05-04T12:33:00Z","ready":true},
  {"profile":"personal","active":false,"account":null,
   "source":"env","ready":false}
]
```

## Files

- `src/commands/auth/status.rs` (new).
- `src/commands/auth/mod.rs` — register `status`.
- `src/util/userinfo.rs` (new) — call `oauth2.googleapis.com/userinfo`
  to resolve email (cached in `store.json` after first call).

## Steps

1. Read `store.json` for profile, parse `expires_in` +
   `fetched_at` to compute remaining validity.
2. If `--check`, build `CalendarHub` and call `calendars.get("primary")`
   — surface 401/403 as actionable error.
3. Resolve account email: cached in `store.json` extra field, else
   call userinfo endpoint once and write back.
4. Render table or JSON depending on `--json` global flag.

## Tests

- Unit: token-expiry math (`fetched_at + expires_in - now`).
- Unit: JSON shape stable (snapshot).
- Integration: missing token → `ready: false` + actionable message.

## Validation

```sh
gcal auth status               # active profile only
gcal auth status --all         # every profile
gcal auth status --all --json  # machine output
gcal auth status --check       # live ping
```

## Out of scope

- Editing token (refresh = P-W1 if needed).
- Token revocation (handled by `logout`, P4).

## Result

_Filled when phase closes._
