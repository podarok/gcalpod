# W7-P0 — Research

## Goal

Survey output styles for terminal calendar tools.

## Sources reviewed

- [insanum/gcalcli](https://github.com/insanum/gcalcli) — `agenda`,
  `calw`, `calm` commands; `--lineart unicode|fancy|ascii` flag.
- [ratatui](https://ratatui.rs/) — TUI calendar widget; overkill
  for one-shot CLI output.
- [comfy-table](https://docs.rs/comfy-table/) — current renderer.
  Supports `ContentArrangement::DynamicFullWidth` (we use it),
  `set_truncation_indicator`, custom border styles.
- [terminal_size](https://docs.rs/terminal_size/) crate — detect
  cols/rows for adaptive layout.
- [unicode-width](https://docs.rs/unicode-width/) — column-width
  calc for non-ASCII strings.

## Findings

Best-practice patterns for CLI calendar:

1. **Day-grouped agenda** — flat list, day separators. Wraps any
   width.
2. **Compact week grid + line art** — needs wide terminal.
3. **Auto-detect terminal width** — adapt format choice.
4. **Highlight today** — bold + marker.
5. **All-day section** — `[ALL DAY]` prefix or separate top section
   per day.
6. **Truncate with `…`** — never wrap mid-cell.

## Result

Logged. Hand-off to P1 for option selection.
