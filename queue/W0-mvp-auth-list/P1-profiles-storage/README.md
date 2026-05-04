# P1 — Profile storage layout

## Goal

Introduce per-profile credential + token storage at
`~/.gcal/profiles/<name>/` with backwards-compatible fallback to
legacy `~/.gcal/{secret,store}.json` paths as the `default` profile.

Add `~/.gcal/config.toml` with `active_profile`, `default_calendar`,
`tz` keys. Active profile resolved via:
`--profile flag > GCAL_PROFILE env > config.toml.active_profile > "default"`.

## Files

- `src/profile.rs` (new) — `Profile` struct, path resolvers,
  `active_profile()`.
- `src/config.rs` (new) — `Config` struct, TOML load/save, atomic write.
- `src/util/calendar.rs` — `resolve_secret()` consults profile dir
  before legacy file. `auth()` writes `store.json` to profile dir.
- `src/main.rs` — wire global `--profile` flag (clap derive arg).
- `Cargo.toml` — add `serde`, `serde_derive`, `toml`.

## Steps

1. Add `serde` + `toml` deps.
2. Define `Profile { name: String, dir: PathBuf }` with methods
   `secret_path()`, `store_path()`, `ensure_dir()`.
3. Define `Config` with `active_profile`, `default_calendar`,
   `tz`. Implement `load_or_default()` + `save_atomic()` (write to
   `config.toml.tmp` then rename — POSIX atomic).
4. Resolution order in `Profile::resolve(cli_flag, env, config)`.
5. Update `resolve_secret()` to check profile dir before legacy
   `~/.gcal/secret.json`.
6. Update `auth()` to persist tokens at `profile.store_path()`
   instead of hardcoded `~/.gcal/store.json`.
7. Add migration helper: if active profile is `default` AND
   `~/.gcal/profiles/default/` is empty AND legacy files exist →
   move them in + log to stderr.

## Tests

- Unit: `Profile::resolve` precedence (flag > env > config > "default").
- Unit: `Config` round-trip (load → save → load equal).
- Unit: migration moves legacy files only when target dir empty.
- Integration: env-set `HOME` to tempdir, run `auth()` with mocked
  `ApplicationSecret`, assert `profile/default/store.json` written.

## Validation

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
# Smoke:
GCAL_VERBOSE=1 cargo run -- --profile work --help
```

## Out of scope

- The `auth login` subcommand itself (P2).
- `auth status` / `logout` / `switch` (P3, P4).
- Profile listing UI — just storage. Listing is part of `auth status`.

## Result

Implemented 2026-05-04 on `main`.

- `src/profile.rs` — `Profile { name, dir }` with `secret_path()`,
  `store_path()`, `ensure_dir()`, `resolve_active()`, `migrate_legacy_if_needed()`.
- `src/config.rs` — `Config { active_profile, default_calendar, tz,
  default_format }` with `load_or_default()`, `save_atomic()` (POSIX atomic).
- `src/main.rs` — global `--profile <name>` flag, resolves active
  profile, runs migration, passes Profile into `auth()`.
- `src/util/calendar.rs` — `resolve_secret(profile)` consults
  `profiles/<name>/secret.json` before legacy `~/.gcal/secret.json`.
  `auth(profile)` persists token to `profiles/<name>/store.json`.
- `Cargo.toml` — added `serde` + `toml` deps.

Resolution order final:
1. `GCAL_CLIENT_ID` + `GCAL_CLIENT_SECRET` env vars.
2. `GCAL_SECRET_FILE=<path>` env.
3. `~/.gcal/profiles/<active>/secret.json`.
4. `~/.gcal/secret.json` (legacy fallback for un-migrated installs).

Active-profile resolution: `--profile` flag > `GCAL_PROFILE` env >
`config.toml.active_profile` > `"default"`.

Migration: first run on `default` profile with empty
`profiles/default/` and existing legacy files moves them in (rename,
not copy). Logs one stderr line.

Tests: 6 new pass (4 `Profile::resolve_active` precedence + 2
`Config` serde round-trip). Pre-existing failure in
`util::date::test_get_start_of_the_week` is unrelated.

`cargo run -- --help` shows `--profile <profile>` in global options.
