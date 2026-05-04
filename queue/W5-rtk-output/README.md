# Wave W5 — RTK output format

Apply [`rtk-ai/rtk`](https://github.com/rtk-ai/rtk) token-killer
output conventions to all `gcal` commands and `--help` text.

Goal: 60-90% token reduction on routine operations, LLM-pipeline
friendly, owner-approved style decisions before implementation.

**Independent of W0/W1/W2** — pure presentation refactor.

## Phases

| Phase | Goal | Status |
|---|---|---|
| [P0-research](P0-research/) | Capture RTK design principles + collect example transformations. | done |
| [P1-options-proposal](P1-options-proposal/) | Draft 3 design options (lite / full / ultra) + owner picks one. | pending — needs owner sign-off |
| [P2-implement](P2-implement/) | Apply chosen option to all subcommands + `--help`. Add `-u/--ultra-compact` flag. | blocked by P1 |

## Done when

- Every command's default output passes token-density review
  (1-3 line summaries replace verbose blocks).
- `--help` text trimmed, ASCII-only, no decorative borders.
- `-u/--ultra-compact` flag (or equivalent) shipped where
  applicable.
- Recovery metadata (full-log path) printed on failures.

## Decisions

See [`DECISIONS.md`](DECISIONS.md).
