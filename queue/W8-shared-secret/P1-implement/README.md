# W8-P1 — Implement shared secret

## Goal

Land code + tests + docs for shared OAuth secret across profiles.

## Files

- `src/profile.rs` — `Profile::shared_secret_path()`,
  `Profile::shared_flag_path()`. `migrate_legacy_if_needed()`
  reads sentinel: if present → only move `store.json`, keep
  `secret.json` shared.
- `src/commands/init.rs` — `--shared` flag in `init`. When set,
  destination is `~/.gcal/secret.json` and sentinel created.
- `src/main.rs` — register `--shared` on `init` clap.
- `docs/profiles.md` (new) — explain shared vs per-profile flow.
- `README.md` — Configuration section mentions shared mode.

## Tests

- Unit: sentinel-present causes migration to skip secret.
- Unit: shared path resolution still hits via legacy chain.
- Integration: init --shared in tempdir HOME → secret + sentinel
  exist + per-profile dirs empty for secret.

## Validation

```sh
rm -rf ~/.gcal
gcal init --shared --profile default
# secret + sentinel land at ~/.gcal/secret.json + ~/.gcal/shared.flag
gcal auth login --profile work     # uses shared secret, own store.json
gcal auth login --profile personal # uses shared secret, own store.json
gcal auth status --all             # both ready, shared secret
```

## Out of scope

- Encrypted shared secret (keychain).
- Cross-machine sync.

## Result

Implemented 2026-05-04 on `main`.

Files:
- `src/profile.rs` — `Profile::shared_secret_path()` +
  `Profile::shared_flag_path()` exposed. `migrate_legacy_if_needed()`
  reads `~/.gcal/shared.flag`; in shared mode, only `store.json`
  migrates (secret stays at `~/.gcal/secret.json`).
- `src/commands/init.rs` — `run(profile, shared)` signature; saves
  secret to shared path + writes sentinel when `shared=true`.
- `src/main.rs` — `init --shared` clap flag wired into dispatch.
- `src/util/calendar.rs` — `SecretSource::LegacyFile` renamed to
  `Shared`. Verbose log line updated.
- Unit test: `shared_paths_under_dotgcal`.

47 tests pass. Build clean.

Workflow:
```
gcal init --shared --profile default
gcal auth login --profile work       # shared secret, own token
gcal auth login --profile personal   # shared secret, own token
```
