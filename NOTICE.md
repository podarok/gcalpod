# NOTICE

`gcalpod` is a derivative work of
[`rust-dd/google-calendar-cli`](https://github.com/rust-dd/google-calendar-cli).

- **Original repository:** https://github.com/rust-dd/google-calendar-cli
- **Fork point commit:** `0dc93e08093c9a1027ebfdd2544a04fc197be60d`
- **Date forked:** 2026-05-04
- **Original license:** Apache License, Version 2.0 (see [`LICENSE-Apache-2.0`](LICENSE-Apache-2.0))
- **Original copyright:** rust-dd organization
- **This fork's license:** PolyForm Noncommercial 1.0.0 + gSL-v1
  Sustainable License Addendum (see [`LICENSE`](LICENSE) and
  [`LICENSE-ADDENDUM.md`](LICENSE-ADDENDUM.md)). Apache 2.0 retained
  for upstream-derived portions per Apache 2.0 §4.

## Substantial modifications

| Date | Change |
|---|---|
| 2026-05-04 | Added env-var (`GCAL_CLIENT_ID`/`GCAL_CLIENT_SECRET`/`GCAL_PROJECT_ID`) + custom-file (`GCAL_SECRET_FILE`) OAuth secret resolution chain. Added `GCAL_VERBOSE=1` source logging. |
| 2026-05-04 | Added `.github/FUNDING.yml` sponsorship configuration. |
| 2026-05-04 | Scaffolded `queue/` work plan (anatomy mirror of [`gogs-cli-queue`](https://github.com/ITCare-Company/template_for_agents/tree/main/process-knowledge-base/gogs-cli-queue)). 4 waves, 23 phases, 7 cross-cutting epics covering multi-profile auth, output formats (`--format json/tsv/csv/raw`), CLI restructure, tests, docs, fork detach. |
| 2026-05-04 | Renamed package + repository to `gcalpod`. Detached from upstream fork relationship. |
| 2026-05-04 | Removed hardcoded shared-OAuth fallback secret (security fix); rewrote git history via `git-filter-repo` to purge it; force-pushed `main`. |
| 2026-05-04 | Added shared OAuth secret support (`gcal init --shared`), hybrid grid/agenda renderer, RTK failure-first error tee, `--verbose` flag, conky output, man page (`--gen-man`), token refresh subcommand. |
| 2026-05-04 | Relicensed original modifications under PolyForm Noncommercial 1.0.0 + gSL-v1 Sustainable License Addendum. Upstream-derived portions remain available under Apache 2.0 (see `LICENSE-Apache-2.0`). |

Original Apache 2.0 portions remain available under Apache 2.0.
Original modifications by the licensor are released under PolyForm
Noncommercial 1.0.0 plus the gSL-v1 addendum (see
[`LICENSE`](LICENSE) and [`LICENSE-ADDENDUM.md`](LICENSE-ADDENDUM.md)).

## Acknowledgements

Thanks to the upstream `rust-dd/google-calendar-cli` authors for the
original implementation. This derivative builds on top of their
work; please consider supporting the upstream project as well.
