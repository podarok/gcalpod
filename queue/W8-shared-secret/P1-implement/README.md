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

_Filled when shipped._
