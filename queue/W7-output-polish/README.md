# Wave W7 — `gcal list` table output polish

Live `gcal list` table output (current week grid) has rendering
issues:
- Stray `|` characters from wrap escape.
- Narrow column wrap garbles long titles.
- All-day events not rendered.
- No empty-day placeholder.
- No "today" / "now" indicator.
- Width not auto-detected.

Goal: pick an improved layout style + ship it before v1.0.0.

## Phases

| Phase | Goal | Status |
|---|---|---|
| [P0-research](P0-research/) | Survey gcalcli, ratatui calendar widget, comfy-table best practices. | done |
| [P1-options](P1-options/) | 3 layout options + owner picks. | pending — owner gate |
| [P2-implement](P2-implement/) | Apply chosen option. | blocked by P1 |

## Done when

- `gcal list` (default flag set) renders cleanly on 80 / 120 / 200
  column terminals.
- All-day events visible.
- Empty days marked.
- Today's column / row visually distinguished.
- No stray `|` / wrap bugs.

## Decisions

See [`DECISIONS.md`](DECISIONS.md).

## Out of scope

- Interactive TUI navigation (ratatui-style).
- Color themes beyond default green/blue accent.
- Multi-calendar overlay.
