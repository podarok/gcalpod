# W6 Decisions

Append-only.

---

## 2026-05-04 — Cut v1.0.0 directly (skip 0.x rc)

Codebase has stabilised across W0+W1+W3+W4+W5-step-1. No major
behaviour change pending. Breaking-change items (W5-P2 step-2/3,
W2-P1 formatters) are additive, not breaking.

Per W3 detach + rename, the project shifted from "fork in early
stages" to a standalone tool with consistent behaviour. 1.0.0
declares the feature surface stable for the user-visible commands
delivered.

Future 1.x increments:
- 1.1.0: --help trim + `-u` ultra-compact flag (W5 step-2/3).
- 1.2.0: formatters (W2-P1).
- 1.3.0: man page generation (W2-P2).

## 2026-05-04 — crates.io publish deferred

Not blocking v1.0.0 tag. Separate owner decision. Required steps if
greenlit later:
- `cargo login <api-key>`.
- Verify name `gcalpod` still free on crates.io.
- `cargo publish --dry-run` to inspect.
- `cargo publish`.
