# W3-P1 — Name selection

## Goal

Pick repo + crate name. Verify availability + non-collision with
trademarks.

## Constraints

- GitHub: unique in `podarok/` namespace.
- Cargo: lowercase, hyphen / underscore separated, valid crate name.
- Avoid Google trademark territory ("google", "g-suite",
  "calendar" alone with implied endorsement).
- Short (≤ 12 chars) so `cargo install <name>` is ergonomic.
- Binary name should match or be obviously derived.

## Candidates

| Name | GitHub free? | Cargo free? | Notes |
|---|---|---|---|
| `gcal` | TBD (probably taken) | TBD | shortest, generic |
| `gcal-cli` | TBD | TBD | descriptive |
| `gcal-rs` | TBD | TBD | -rs suffix common in Rust ecosystem |
| `kalendar` | TBD | TBD | unique, KDE collision? |
| `gcalrs` | TBD | TBD | rust-flavored |
| `gctl` | TBD | TBD | "gcal control" |
| `kal` | TBD | TBD | very short, probably taken |

## Steps

1. Check GitHub: `gh api users/podarok/repos --jq '.[].name'` —
   reject anything already in namespace.
2. Check crates.io for each: `cargo search <name>`.
3. Quick web search for "<name> command line" — surface trademark
   collisions (e.g., calendar apps with same name).
4. Owner picks one.

## Decision criteria (final tie-break)

- Discoverability (Google search returns this tool, not noise).
- Pronounceable.
- No collision with existing common command name (`kal`, `cal`
  already exist on Unix → bad).

## Out of scope

- Logo / icon design.
- Domain registration.

## Result

_Filled when name picked + verified._
