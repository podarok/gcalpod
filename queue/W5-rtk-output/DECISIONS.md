# W5 Decisions

Append-only.

---

## 2026-05-04 — RTK research findings (W5-P0)

Source: https://github.com/rtk-ai/rtk

Core principles:
- **Smart filter** — remove boilerplate (timestamps, banners, version
  headers).
- **Grouping** — aggregate similar items (lint by rule/file, git
  one-line commits).
- **Truncation** — cut redundancy (test passes summarized, failures
  detailed).
- **Deduplication** — collapse repeated lines with counts.
- **Failure-first** — `FAILED: 2/15 tests` style; passing tests
  omitted; failing assertions detailed.
- **Recovery metadata** — `[full output: <path>]` reference on
  failures so user can drill in.
- **Ultra-compact mode** — `-u, --ultra-compact` flag swaps to
  ASCII icons, inline format.
- **Line-length** not specified; aggressive compression (1-3 line
  summaries replacing 15-200+ line outputs).
- Design prioritizes LLM context efficiency over human readability.

Open questions for P1:
1. Apply RTK style to default output, or keep verbose default and
   gate compact behind `-u`?
2. Which commands get RTK treatment first (auth status / list /
   import)?
3. `--help` output: trim flag descriptions, drop usage examples,
   collapse long enums?
4. Do we ship our own `rtk` binary integration, or just borrow the
   formatting conventions?
5. Failure-first split: where do non-failure warnings (e.g.
   "no events in range") live?

Hand-off to P1: present 3 options to owner, owner picks one.
