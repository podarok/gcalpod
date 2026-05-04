# W7-P1 — Options

## Layout options

### Option A — Day-grouped agenda everywhere

```
Mon 4 May 2026 ─────────────────────────────
  10:00-11:00  Weekly planning
  16:00-17:30  GCIP2 Module 0: Programme Orientation

Tue 5 May 2026 ─────────────────────────────
  10:00-10:15  ITCare Daily Stand up
  16:00-17:30  GCIP2 Module 1: Goal Setting & KPIs
   …
```

Pro: clean on every width; single renderer; trivial to wrap long
titles. Con: loses the "look at the week at a glance" feel.

### Option B — Improved week grid (unicode line-art) + width adapt

Same week grid, but:
- Pull terminal width via `terminal_size` crate; fall back to 120.
- `unicode-width` for non-ASCII alignment.
- Truncate cells with `…` instead of wrap.
- Highlight today's column header (bold + bg accent).
- All-day events in a top "All-day" sub-row.
- Empty cells show `·`.

Pro: at-a-glance week view. Con: still cramped on < 100 cols;
single-day view limited.

### Option C (recommended) — Hybrid

- Range ≤ 7 days AND terminal cols ≥ 100 → improved week grid (B).
- Range > 7 days OR cols < 100 → day-grouped agenda (A).
- `--style grid|agenda|auto` flag for explicit override.
- `--lineart unicode|fancy|ascii` flag (default `unicode`).

Pro: best per scenario. Con: two renderers to maintain.

## Comparison

| Aspect | A | B | C |
|---|---|---|---|
| Default range | day-grouped | week grid | adaptive |
| 80-col terminal | OK | cramped | falls to A |
| > 7 day range | OK | breaks | falls to A |
| Today highlight | row marker | column bold | both |
| All-day events | rendered | sub-row | both |
| Implementation effort | small | medium | medium |
| Maintenance | one renderer | one renderer | two renderers |

## Recommended pick

**Option C** — hybrid. Owner picks A / B / C in chat (default = C).

## Out of scope

- Color themes beyond default green/blue accent.
- Interactive TUI.
- Conky / Tera template formatters (W2-P1).

## Result

Owner picked **Option C** on 2026-05-04 ("С"). Hybrid renderer with
`--style auto|grid|agenda` + `--lineart unicode|fancy|ascii`.
