# W3-P3 — Preserve attribution

## Goal

Comply with Apache 2.0 §4: retain LICENSE, document changes, preserve
copyright + attribution notices.

## Files

- `LICENSE` — keep verbatim from upstream.
- `NOTICE.md` (new) — origin + changes log.
- `README.md` — add "Based on" line near top.
- `CHANGELOG.md` (new, optional) — explicit changes from fork point.
- Source headers in modified files — Apache 2.0 boilerplate preserved.

## NOTICE.md template

```markdown
# NOTICE

This project is a derivative work of
[`rust-dd/google-calendar-cli`](https://github.com/rust-dd/google-calendar-cli).

- Original repository: https://github.com/rust-dd/google-calendar-cli
- Fork point commit: `<sha>` (date: 2026-04-XX)
- Original license: Apache License, Version 2.0 (see LICENSE)
- Original authors: rust-dd organization

## Substantial modifications

| Date | Change |
|---|---|
| 2026-05-04 | Added env-var + custom-file OAuth secret resolution |
| 2026-05-04 | Restructured CLI into subcommand tree (`auth`, `events`, ...) |
| 2026-05-04 | Multi-profile token storage |
| 2026-05-04 | `--format json/tsv/csv/raw` output for `list` |
| <date>     | <change> |

Modifications are released under the same Apache License 2.0.
```

## Steps

1. Determine fork-point commit on upstream `main`.
2. Write `NOTICE.md` with template above.
3. Add `## Acknowledgements` section to `README.md` linking
   `NOTICE.md`.
4. Audit modified source files: ensure no upstream copyright lines
   stripped accidentally.

## Tests

- Manual: `diff` upstream `LICENSE` vs derivative — must be identical.
- `grep -r 'rust-dd' .` — confirms attribution still surfaces in
  README + NOTICE.

## Validation

- `LICENSE` byte-identical to upstream.
- `NOTICE.md` present + lists at least one substantial modification.
- README "Acknowledgements" section links to upstream.

## Out of scope

- Reaching out to upstream maintainers for explicit blessing
  (Apache 2.0 doesn't require it; courtesy email optional).

## Result

_Filled when attribution committed._
