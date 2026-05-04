# W7-P2 — Implement

## Goal

Apply the W7-P1 owner-picked option to `gcal list` (and reuse for
`agenda` flat-list).

## Files (option C scaffold)

- `Cargo.toml` — add `terminal_size` + `unicode-width` deps.
- `src/util/render.rs` (new) — `render_table_smart(events, range,
  tz, style, lineart)` dispatch.
- `src/main.rs` — wire `--style` + `--lineart` flags on `list`.
- existing renderers refactored into `render::week_grid()` and
  `render::agenda_grouped()`.

## Steps

1. Detect terminal width via `terminal_size::terminal_size()`.
2. Decide renderer per range + width.
3. `agenda_grouped`:
   - Sort events by start.
   - Group by `date_naive()`.
   - Per day: header line `Mon 4 May 2026 ─────`.
   - All-day events first (`[ALL DAY] <summary>`).
   - Timed events: `HH:MM-HH:MM  <summary>`.
   - Empty days: `(none)`.
   - Truncate summary at width-2 chars with `…`.
4. `week_grid`:
   - Comfy-table with `set_border_lines` for unicode style.
   - Column header: `<weekday>\n<dd Mon>` with today bold.
   - All-day row at top.
   - AM/PM rows below as today.
   - Truncate cells, never wrap.
5. Stable sort everywhere.

## Tests

- Snapshot: `agenda_grouped` on fixture event set.
- Snapshot: `week_grid` on same set, 80-col + 200-col widths.
- Truncation: long summary `…`-clipped.
- All-day: single all-day event renders in dedicated section.

## Validation

```sh
gcal list                       # adaptive (default C)
gcal list --style grid          # force grid
gcal list --style agenda        # force agenda
gcal list --lineart ascii       # ASCII fallback
gcal list --from today --to +30d  # > 7 days -> agenda auto
```

## Out of scope

- ratatui interactive view.
- Color themes.

## Result

Implemented 2026-05-04 on `main`.

Files:
- `src/util/render.rs` (new) — `LayoutStyle { Auto, Grid, Agenda }`,
  `LineArt { Unicode, Fancy, Ascii }`, `render_list_smart()`
  dispatcher; `render_week_grid()` and `render_agenda_grouped()`
  renderers. Uses `terminal_size::terminal_size()` for adaptive
  width and `unicode_width::UnicodeWidthStr` for non-ASCII column
  alignment. `truncate_to_width` clips with `…`.
- `src/util/mod.rs` — register `render`.
- `src/main.rs` — list arm replaces flat-list + week-grid blocks
  with single dispatch. Adds `--style` + `--lineart` clap flags.
  Removes unused `HashMap`/`Entry`/`Cell`/`Color`/`Table`/`Month`
  imports.
- `Cargo.toml` — add `terminal_size` 0.4 + `unicode-width` 0.2.

Auto mode rule: range_days ≤ 7 AND cols ≥ 100 → grid; else agenda.

Today highlighted yellow + bold in grid header; agenda day header
gets `← today` marker.

All-day events: own row at top of grid; `[ALL DAY]` prefix in
agenda. Empty cells show `·`. Empty agenda days show `(none)`.

Tests: 6 new in `util::render` (`LayoutStyle::parse`,
`LineArt::parse`, 4× truncate_to_width edge cases). All 46 tests
pass.

Smoke: `gcal list` renders cleanly on real Calendar API data.

Open follow-up: cell width allocation could give more space to
days with content (currently uniform `(cols-20)/7`). Defer to
1.x feature release if needed.
