# W3-P0 — License audit

## Goal

Confirm upstream `LICENSE` text + draft compliance plan for derivative.

## Findings

- License: Apache License 2.0 (verified against upstream `LICENSE`
  file in `rust-dd/google-calendar-cli`).
- No `NOTICE` file in upstream — derivative is free to add its own.
- Author of upstream: `rust-dd` GitHub org.

## Compliance checklist for derivative

- [x] Retain `LICENSE` file unchanged.
- [ ] Add `NOTICE.md` documenting:
  - Original project URL
  - Original commit hash at fork point
  - Date forked
  - Substantial modifications (with date + author)
- [ ] Add per-source-file headers preserving Apache 2.0 reference.
  (Section 4(c) of Apache 2.0: "You must retain ... all copyright,
  patent, trademark, and attribution notices".)
- [ ] State changes prominently (Section 4(b)): summary in
  `CHANGELOG.md` or top of `README.md`.

## Files

- `LICENSE` — keep as-is.
- `NOTICE.md` — to be created in P3.
- `README.md` — add "Forked from" / "Based on" line in P5.

## Out of scope

- Other dependency licenses (Cargo will surface those at build time;
  not our compliance burden for fork detach).

## Result

License confirmed Apache 2.0. Compliance plan above. Hand off to
P1-new-name-selection.
