# Epic 07 — Detach fork + rename project

Sever the GitHub fork relationship from `rust-dd/google-calendar-cli`
and rename the project per GitHub naming rules + license obligations.

Goal: stand alone as `podarok/<new-name>` with proper attribution
to the upstream original.

## Scope

- License audit of upstream `LICENSE` to determine attribution
  requirements.
- New name selection (must be unique in `podarok` namespace +
  Cargo crate name available on crates.io if publishing).
- Detach mechanism (GitHub Settings "Leave fork network" or new
  repo + push if detach unavailable).
- Preserve attribution: keep upstream LICENSE file + add NOTICE
  with provenance (original commit hash, upstream URL, date forked).
- Rename Cargo package + binary if name changes (matches new repo).
- Update all internal references (README links, queue references,
  docs URLs).

## Phases

Implementation lives in [`W3-detach-rename/`](../W3-detach-rename/):

- P0-license-audit
- P1-new-name-selection
- P2-detach-or-recreate
- P3-attribution-preserve
- P4-cargo-rename
- P5-update-references

## Done when

- `gh repo view podarok/<new-name>` shows `isFork: false` (or new
  repo created without fork relation).
- `LICENSE` retained from upstream + `NOTICE.md` documents origin.
- `Cargo.toml` reflects new name; `cargo build` clean.
- Every `README.md`, `docs/`, `queue/` reference updated.
- Old repo `podarok/google-calendar-cli` archived (read-only) or
  redirected to new repo.

## Out of scope

- Publishing to crates.io — separate decision (W2 follow-up).
- Logo / branding redesign — naming only.
- Trademark research beyond GitHub uniqueness check.
