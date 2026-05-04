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

Implemented 2026-05-04 on `main`.

Surface delivered: `gcal auth status [--all] [--check] [--show-token]`.
JSON output deferred to W0-P6 (`--format` is a list-command flag, not
a global flag yet).

Files:
- `src/commands/auth/status.rs` (new) — `StatusArgs`, `run()`,
  `list_all_profiles()`, `report_profile()`. Reads `store.json`
  via `serde_json` for offline access/refresh-token presence check.
- `src/commands/auth/mod.rs` — export `status`.
- `src/main.rs` — register `status` clap subcommand + dispatch.
- `Cargo.toml` — add `serde_json`.

Output:
```
Profile: default (active)
  Secret:  /Users/.../.gcal/profiles/default/secret.json
  Scopes:  https://...auth/calendar, https://...auth/calendar.events, ...
  Token:   access=yes refresh=yes (path: /Users/.../.gcal/profiles/default/store.json)
  State:   ✓ ready (offline check)
```

`--all` enumerates `~/.gcal/profiles/` (sorted). Marks active profile
with ` (active)`. `--check` builds authenticator and pings
`hub.calendar_list().list().max_results(1)`. `--show-token` prints
bearer with stderr warning.

Smoke note: first `auth status` run triggered legacy migration
(`~/.gcal/{secret,store}.json` → `~/.gcal/profiles/default/`),
confirming W0-P1 migration path.
