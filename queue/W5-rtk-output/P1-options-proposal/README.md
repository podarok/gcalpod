# W5-P1 — Options proposal (owner gate)

## Goal

Draft 3 concrete design options for RTK-style output; owner picks
one before P2 implementation.

## Options

### Option A — opt-in compact (`-u/--ultra-compact` flag)

Default output stays verbose / human-readable as today. Add `-u`
flag globally that switches every command to RTK-style 1-3 line
output. `--help` unchanged.

Pro: zero risk for existing flows; opt-in for LLM/scripting use.
Con: doesn't realize token savings by default.

### Option B — RTK-by-default, `-v/--verbose` to expand

Flip default: every command emits compact RTK output. Add `-v`
to bring back verbose. `--help` text trimmed (drop examples + long
enum value lists; keep one-line description per flag).

Pro: maximum token savings in normal use. Aligns with RTK ethos.
Con: behaviour change visible to existing users; potential surprise
in CI logs.

### Option C — Hybrid (compact errors + summaries, verbose body)

Failure-first principle applied unconditionally: errors get
RTK-style highlight + recovery-metadata path. Successful command
bodies remain verbose by default. Add `-u` to also compact bodies.
`--help` trimmed always.

Pro: failure-first wins shipped in default; verbose body preserved
for human read.
Con: still leaves token savings on the table for piped consumption.

## Comparison table

| Aspect | Option A | Option B | Option C |
|---|---|---|---|
| Default output | verbose | compact | verbose body, compact errors |
| `-u/--ultra-compact` | enables compact | enables ultra | enables compact body |
| `-v/--verbose` | n/a | restores verbose | n/a |
| `--help` text | unchanged | trimmed | trimmed |
| Risk to existing users | none | high | low |
| Token savings (default) | none | high | partial |
| Effort estimate | small | medium | medium |

## Recommended pick

**Option C** — hybrid. Trims `--help` (cheap win), unconditional
failure-first + recovery metadata, verbose body preserved, opt-in
compact via `-u`. Best balance of compatibility + token savings.

## Out of scope

- Wrapping `gcal` inside `rtk` runtime — separate decision.
- JSON-stream / NDJSON output (already covered by `--format json`
  in W0-P6).
- Per-command custom formatters — defer to W2-P1.

## Result

_Filled when owner picks an option._
