# W1-P1 — `gcal config get/set/path`

## Goal

Manage `~/.gcal/config.toml` from CLI without a text editor. Mirrors
`git config` UX. Used to set `default_calendar`, `tz`,
`active_profile`, `output_format`.

## Surface

```
gcal config get <key>
gcal config set <key> <value>
gcal config unset <key>
gcal config list
gcal config path                 # print absolute path to config.toml
```

Known keys (validated):
- `active_profile` — string (must exist as profile dir).
- `default_calendar` — string (calendar id).
- `tz` — IANA timezone (validated via chrono-tz).
- `default_format` — `table|json|tsv|csv|raw`.

Unknown keys allowed but logged as warning (forward-compat).

## Files

- `src/commands/config/{mod,get,set,list,path}.rs`.
- `src/config.rs` (from W0-P1) — add `set/get/unset` methods.

## Tests

- Round-trip: set → list → get equal.
- Validation: bad TZ rejected with helpful error.

## Validation

```sh
gcal config set tz Europe/Kyiv
gcal config get tz                  # → Europe/Kyiv
gcal config path                    # → /Users/.../.gcal/config.toml
gcal config list                    # all keys
```

## Out of scope

- Per-profile config overrides — global only for now.

## Result

Implemented 2026-05-04 on `main`.

Surface: `gcal config get/set/unset/list/path`. Single-file impl
(`src/commands/config_cmd.rs`) instead of per-subcommand split
to keep dispatch flat — module split deferred to
[`epic-01-cli-restructure`](../../epic-01-cli-restructure/README.md).

Files:
- `src/commands/config_cmd.rs` (new) — `ConfigAction` enum,
  `run(action)`, `read_field/write_field/validate`. Validates `tz`
  via `chrono_tz::Tz` parse and `default_format` via
  `OutputFormat::parse`.
- `src/commands/mod.rs` — register `config_cmd`.
- `src/main.rs` — register `config` clap subcommand with five
  sub-subcommands; dispatch before `calendar::auth()`.

Known keys (validated):
- `active_profile` — string.
- `default_calendar` — string (no validation; calendar id).
- `tz` — IANA timezone (validated).
- `default_format` — `table|json|tsv|csv|raw` (validated).

`get` matches `git config --get` semantics: empty stdout when unset.
Unknown keys rejected with explicit error listing the four valid keys
(stricter than the original plan's "warn" — easier to review).

Smoke verified:
- `gcal config list` shows all four keys (empty values for unset).
- `gcal config set tz Europe/Kyiv` → `tz = Europe/Kyiv`.
- `gcal config get tz` → `Europe/Kyiv`.
- `gcal config set tz NotARealTz` → "invalid IANA timezone" error.
