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

_Filled when phase closes._
