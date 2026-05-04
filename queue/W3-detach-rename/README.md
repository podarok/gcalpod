# Wave W3 — Detach fork + rename

Sever GitHub fork relationship + rename per GitHub rules + Apache 2.0
attribution requirements.

**Independent of W0/W1/W2** — can run in parallel with code work.

## Phases

| Phase | Goal | Status |
|---|---|---|
| [P0-license-audit](P0-license-audit/) | Confirm Apache 2.0 obligations + draft NOTICE.md text. | pending |
| [P1-new-name-selection](P1-new-name-selection/) | Pick new repo + Cargo name; verify availability. | pending |
| [P2-detach-or-recreate](P2-detach-or-recreate/) | Detach via GitHub Settings or recreate as standalone repo. | pending |
| [P3-attribution-preserve](P3-attribution-preserve/) | Add NOTICE.md, update LICENSE comment headers, retain upstream copyright. | pending |
| [P4-cargo-rename](P4-cargo-rename/) | Rename `Cargo.toml` `name`, binary, install path. | pending |
| [P5-update-references](P5-update-references/) | README, docs, queue/, .github/ links + workflows updated to new name. | pending |

## Done when

- New repo isn't a GitHub fork of `rust-dd/google-calendar-cli`.
- `LICENSE` retained verbatim. `NOTICE.md` documents origin.
- `Cargo.toml` reflects new name. `cargo build --release` clean.
- All references in `README.md`, `docs/`, `queue/` use new name.
- Old `podarok/google-calendar-cli` redirects or archived.

## Decisions

See [`DECISIONS.md`](DECISIONS.md).

## Out of scope

- crates.io publishing.
- Trademark search beyond GitHub uniqueness.
