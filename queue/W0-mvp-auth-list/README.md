# Wave W0 — MVP: auth + list (stm32 use case)

Smallest surface that pulls events for stm32 firmware planning:
multi-profile auth (`login`/`status`/`logout`/`switch`), `list` with
date range + JSON output, `calendars list` for non-primary picks.

**Blocks W1.**

## Phases

| Phase | Goal | Spec | Status |
|---|---|---|---|
| [P0-prelude-env-resolution](P0-prelude-env-resolution/) | Env-var + custom-file OAuth resolution; `GCAL_VERBOSE`. | done — see `feat/custom-oauth-config` | done |
| [P1-profiles-storage](P1-profiles-storage/) | `~/.gcal/profiles/<name>/{secret.json,store.json}` layout + legacy migration. | spec | pending |
| [P2-auth-login](P2-auth-login/) | `gcal auth login [--profile] [--no-browser] [--scopes]`. | spec | pending |
| [P3-auth-status](P3-auth-status/) | `gcal auth status` — email, scopes, token expiry, source. | spec | pending |
| [P4-auth-logout-switch](P4-auth-logout-switch/) | `gcal auth logout [--profile|--all]` + `gcal auth switch <profile>`. | spec | pending |
| [P5-list-range](P5-list-range/) | `gcal list --from <date> --to <date> [--calendar <id>]`. | spec | pending |
| [P6-list-json](P6-list-json/) | `--format json|tsv|csv|raw|table` for `list`. JSON for jq, TSV/CSV for spreadsheet, raw for full upstream API fields. | spec | pending |
| [P7-calendars-list](P7-calendars-list/) | `gcal calendars list` — id / summary / primary flag. | spec | pending |

## Done when

- `gcal auth login --profile work` succeeds for a fresh OAuth project.
- `gcal auth status` shows the active profile + email + token expiry.
- `gcal auth switch personal` → `gcal list --from 2026-05-01 --to
  2026-05-31 --json` returns events as JSON without re-prompting.
- Legacy users with only `~/.gcal/secret.json` keep working unchanged.

## Decisions

See [`DECISIONS.md`](DECISIONS.md).

## Out of scope

- `init` interactive wizard (W1-P0).
- Event modification (`add` polish, `edit`, `delete`) — W1-P3.
- ICS import — W1-P4.
- Conky / template formatters — W2-P1.
- Man page generation — W2-P2.
