# W3-P5 — Update references

## Goal

Sweep every internal reference to old name / old repo URL. Leave
attribution to upstream intact (those stay).

## Targets

- `README.md` — install instructions, badges, links.
- `docs/*.md` — internal cross-links.
- `queue/**/*.md` — anatomy references; update repo paths but keep
  upstream as anatomy source.
- `.github/workflows/*.yml` — CI workflow names + cargo install
  paths.
- `Cargo.toml` `[package]` metadata.
- Source comments referencing old repo URL.

## Steps

1. `grep -rn 'google-calendar-cli\|rust-dd' --include='*.md'
   --include='*.yml' --include='*.toml' --include='*.rs'` — surface
   every hit.
2. Categorize:
   - Drop: stale references to old fork URL.
   - Keep: upstream attribution in NOTICE.md / LICENSE comments.
   - Update: install paths, repository URL.
3. Apply edits. One commit per file group (`docs:`, `chore:`, `ci:`).

## Tests

- `grep -rn 'podarok/google-calendar-cli'` — should be empty post-sweep.
- `grep -rn 'rust-dd'` — should appear ONLY in NOTICE.md, README
  attribution section, and LICENSE-related files.

## Validation

```sh
gh repo view podarok/<new-name> --json description,url   # matches
cargo build --release
gcal --help                                              # OK
```

## Out of scope

- External references (other people's READMEs, blog posts) — out
  of our control.
- Search engine cache invalidation.

## Result

_Filled when sweep committed._
