# P2 — `gcal auth login`

## Goal

Add explicit OAuth login subcommand with profile + scope control.
Replaces implicit auth-on-first-call behavior with a deliberate flow,
mirroring `gh auth login` UX.

## Surface

```
gcal auth login [--profile <name>] [--scopes <csv>] [--no-browser] [--reauth]
```

Flags:
- `--profile <name>` — target profile (default: active).
- `--scopes <csv>` — override scopes; default = full read/write
  Calendar.
- `--no-browser` — print verification URL + paste code (for
  headless / WSL).
- `--reauth` — force flow even if cached token still valid.

## Files

- `src/commands/auth/login.rs` (new) — flow handler.
- `src/commands/auth/mod.rs` (new) — clap subcommand `auth`.
- `src/main.rs` — register `auth` subcommand.
- `src/util/calendar.rs` — extract `build_authenticator()` so
  `login` can call it without `CalendarHub` construction.

## Steps

1. Scaffold `commands/auth/{mod,login}.rs` with clap derive.
2. Implement `login()`: resolve profile → resolve secret →
   `InstalledFlowAuthenticator::builder()` →
   `persist_tokens_to_disk(profile.store_path())` →
   `auth.token(scopes).await`.
3. On `--no-browser`, switch to
   `InstalledFlowReturnMethod::Interactive` (paste code).
4. On `--reauth`, delete `store.json` first.
5. Print success line: profile, email (from `userinfo` API),
   token expiry.
6. Migration on first run: see P1 migration helper.

## Tests

- Snapshot: `gcal auth login --help` (clap output).
- Integration: mock OAuth server + `--scopes` plumbing assertion.

## Validation

```sh
gcal auth login --profile work
# Browser opens, consent, success line printed
gcal auth login --profile work --reauth
# Forces flow even though token valid
gcal auth login --profile headless --no-browser
# Prints URL + waits for paste
```

## Out of scope

- `status` / `logout` / `switch` — separate phases.
- `auth refresh` / `auth token` — W1 follow-up if needed.
- Multi-scope incremental authorization — keep simple full set
  for MVP.

## Result

_Filled when phase closes._
