# Wave W4 — README refresh

Rebuild top-level `README.md` to reflect actual status of completed
work. Currently README still describes early-fork state — needs to
match the reality: gcalpod standalone, env-var auth resolution shipped,
queue scaffolded, fork detached, secret history purged.

**Independent of W0/W1/W2/W3** — pure documentation refresh.

## Phases

| Phase | Goal | Status |
|---|---|---|
| [P0-status-driven-readme](P0-status-driven-readme/) | Rebuild README from current main: status section, command summary, configuration, roadmap link, sponsorship, attribution. | pending |

## Done when

- README header reflects new name + standalone status.
- "Status" section lists completed work + active queue path.
- Command summary table matches current `gcal --help`.
- Configuration section documents env-var resolution chain.
- Roadmap section links to `queue/INDEX.md`.
- Sponsorship section linked from `.github/FUNDING.yml`.
- Attribution section links to `NOTICE.md`.

## Decisions

See [`DECISIONS.md`](DECISIONS.md).
