# Wave W6 — Stable release v1.0.0

Cut a stable release once W0 + W1 + W2-P0 + W3 + W4 + W5-P2-step-1
have landed and the binary covers the daily use case end-to-end.

## Phases

| Phase | Goal | Status |
|---|---|---|
| [P0-changelog](P0-changelog/) | Write `CHANGELOG.md` in Keep-a-Changelog style. | pending |
| [P1-version-bump](P1-version-bump/) | `Cargo.toml` 0.1.0 → 1.0.0. | pending |
| [P2-preflight](P2-preflight/) | `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test --all`, smoke `gcal --help`. | pending |
| [P3-tag-and-release](P3-tag-and-release/) | `git tag v1.0.0` + `gh release create` with notes. | pending |

## Done when

- `Cargo.toml` reports `version = "1.0.0"`.
- Release tag `v1.0.0` exists in `podarok/gcalpod`.
- GitHub release page has the changelog.
- Pre-flight gates green.
- crates.io publish **deferred** until owner explicit ok (separate
  decision; not blocking 1.0.0).

## Out of scope

- W2-P1 formatters (conky / Tera).
- W2-P2 man page.
- W5-P2 step-2 / step-3 (--help trim, -u flag).
- crates.io publish.
- Homebrew formula.

These follow as 1.x feature releases, not blockers for 1.0.0.

## Decisions

See [`DECISIONS.md`](DECISIONS.md).
