# W4-P0 — Status-driven README

## Goal

Rewrite top-level `README.md` so each section maps to actual code
state on `main`, not aspirational early-fork text.

## Sections (target)

1. **Header** — name (`gcalpod`), tagline, badge, attribution line.
2. **Status** — what's shipped, what's in queue. Links to
   `queue/INDEX.md`.
3. **Installation** — current path: `git clone podarok/gcalpod`,
   `cargo build --release`, `cargo install --path . --locked`.
4. **First-time auth setup** — own OAuth project required (no
   shared fallback). Link to `docs/custom_auth.md`.
5. **Usage** — only the commands that exist today: top-level
   quick-add, `add`, `list` (current week). Mark `--from/--to/--format`
   as roadmap.
6. **Configuration** — env-var resolution chain + file paths.
7. **Roadmap** — link to `queue/INDEX.md` for current waves.
8. **Sponsorship** — Sponsor button explanation + `FUNDING.yml`.
9. **Acknowledgements** — link to `NOTICE.md` + upstream.

## Files

- `README.md` (rewrite).

## Steps

1. Audit current `gcal --help` output → command list.
2. Audit current `auth()` flow → resolution chain doc.
3. Draft README sections per anchors above.
4. Replace `README.md` in one commit.

## Tests

- Manual review against current `main` HEAD.
- Verify all internal links resolve (`queue/INDEX.md`, `NOTICE.md`,
  `docs/custom_auth.md`, `.github/FUNDING.yml`).

## Validation

```sh
gcal --help                           # matches "Usage" section
GCAL_VERBOSE=1 gcal list 2>&1 | head  # matches "Configuration" example
```

## Out of scope

- Asciinema recordings.
- Translation.
- Doc-site (mdbook) — single README only.

## Result

_Filled when committed to main._
