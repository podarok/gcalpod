# W7 Decisions

Append-only.

---

## 2026-05-04 — Research findings (W7-P0)

Sources:
- gcalcli (insanum/gcalcli) — `agenda`, `calw`, `calm` reference.
- ratatui — calendar widget + best practices for TUI.
- comfy-table — current renderer.

Best-practice patterns:

1. **Day-grouped agenda** — flat list grouped by day, horizontal
   separator line per day. Wraps cleanly on any width. Sorted by
   start time within day. (gcalcli `agenda` style.)
2. **Compact week grid + unicode line art** — fits 7 days; needs
   wide terminal. `--lineart unicode|fancy|ascii` for terminal
   compat. (gcalcli `calw` style.)
3. **Ratatui TUI** — interactive navigation; overkill for
   one-shot output.
4. **Conky color sequences** — daemon integration; deferred to
   W2-P1.

Common improvements:
- Auto-detect terminal width (`crossterm::terminal::size()` or
  `terminal_size` crate).
- Highlight "today" with marker + bold.
- Render all-day events distinctly (e.g. `[ALL DAY] Holiday`).
- Empty-day placeholder (`(none)` / blank).
- Sort events by start time within day.
- Truncate long titles at column width with `…`.
- Use `unicode-width` for proper alignment with non-ASCII titles.

## 2026-05-04 — Recommended option

Option C (hybrid): day-grouped agenda for ranges > 7 days OR when
terminal width < 100 cols. Week grid (improved) for ≤ 7 days on
wide terminals. `--lineart` flag (unicode default) propagates to
both renderers.

Owner picks A / B / C in P1.
