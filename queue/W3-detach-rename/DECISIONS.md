# W3 Decisions

Append-only.

---

## 2026-05-04 — License confirmed: Apache 2.0

Upstream `LICENSE` is Apache License 2.0. Permits:
- Derivative works.
- Renaming.
- Commercial use.

Requires:
- Retain LICENSE file in derivative.
- State significant changes (commit history + NOTICE.md sufficient).
- Retain copyright + attribution notices.
- If upstream had a NOTICE file, propagate it. Upstream has no NOTICE
  file → new NOTICE.md created in derivative is purely additive.

No copyleft. No requirement to publish derivative source. No naming
restrictions beyond GitHub uniqueness.

## 2026-05-04 — Detach mechanism

Two paths:

1. **GitHub self-service** (preferred, lossless):
   Settings → "Leave fork network" — detaches the fork relationship,
   keeps history, keeps URL. Available since 2024.
2. **Recreate**: `gh repo create podarok/<new-name> --private` →
   push current `main` history → archive old fork.

Try (1) first. Fall back to (2) if GitHub blocks (e.g., upstream
has dependency network protection).

## 2026-05-04 — Naming guidance

- Must be unique in `podarok/` namespace on GitHub.
- Must be valid Cargo crate name (lowercase, hyphen-separated, alpha-num).
- Should not contain "google" or "google-calendar" trademark issues
  (Google brand guidelines: third-party tools should not imply
  endorsement). `gcal` alone is generic enough.
- Candidates: `gcal`, `gcal-cli`, `kalendar`, `gcal-rs`, `gcalrs`.
- Final pick deferred to P1.
