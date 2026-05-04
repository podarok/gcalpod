# Wave W2 — Polish

`remind`, formatter modes (conky / template), `clap_mangen` man page.

**Blocked by W1.**

## Phases

| Phase | Goal | Status |
|---|---|---|
| [P0-remind](P0-remind/) | `gcal remind <mins> -- <cmd...>` — exec command N minutes before next event. | pending |
| [P1-formatters](P1-formatters/) | Optional formatter modes: conky color sequences, custom Tera template. | pending |
| [P2-man-page](P2-man-page/) | Generate man page via `clap_mangen` at build time; install via `make install`. | pending |

## Done when

- `gcal remind 5 -- terminal-notifier ...` triggers on time.
- Conky users can drop `gcal --format conky` into config.
- `man gcal` shows full subcommand reference.

## Decisions

See [`DECISIONS.md`](DECISIONS.md).
