# NOTICE

`gcalpod` is a derivative work of
[`rust-dd/google-calendar-cli`](https://github.com/rust-dd/google-calendar-cli).

- **Original repository:** https://github.com/rust-dd/google-calendar-cli
- **Fork point commit:** `0dc93e08093c9a1027ebfdd2544a04fc197be60d`
- **Date forked:** 2026-05-04
- **Original license:** Apache License, Version 2.0 (see [`LICENSE`](LICENSE))
- **Original copyright:** rust-dd organization

## Substantial modifications

| Date | Change |
|---|---|
| 2026-05-04 | Added env-var (`GCAL_CLIENT_ID`/`GCAL_CLIENT_SECRET`/`GCAL_PROJECT_ID`) + custom-file (`GCAL_SECRET_FILE`) OAuth secret resolution chain. Added `GCAL_VERBOSE=1` source logging. |
| 2026-05-04 | Added `.github/FUNDING.yml` sponsorship configuration. |
| 2026-05-04 | Scaffolded `queue/` work plan (anatomy mirror of [`gogs-cli-queue`](https://github.com/ITCare-Company/template_for_agents/tree/main/process-knowledge-base/gogs-cli-queue)). 4 waves, 23 phases, 7 cross-cutting epics covering multi-profile auth, output formats (`--format json/tsv/csv/raw`), CLI restructure, tests, docs, fork detach. |
| 2026-05-04 | Renamed package + repository to `gcalpod`. Detached from upstream fork relationship. |

Modifications are released under the same Apache License 2.0.

## Acknowledgements

Thanks to the upstream `rust-dd/google-calendar-cli` authors for the
original implementation. This derivative builds on top of their
work; please consider supporting the upstream project as well.
