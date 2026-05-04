# Epic 01 — CLI restructure

Move `clap` from a single `Command` chain in `main.rs` to a
subcommand tree under `src/commands/`. Required by every wave.

## Scope

Current state: ~232 lines of `main.rs` chain `Command::new("gcal")`
with 2 subcommands (`add`, `list`) + top-level title/date args.

Target: derive-style with `clap::Parser`, organized as:

```
src/
├── main.rs                     # 30 lines: parse + dispatch
└── commands/
    ├── mod.rs
    ├── auth/         # W0-P2..P4
    ├── calendars/    # W0-P7
    ├── events/       # W0-P5..P6, W1-P2..P3, W1-P5
    ├── config/       # W1-P1
    ├── init.rs       # W1-P0
    ├── import.rs     # W1-P4
    └── remind.rs     # W2-P0
```

## Done when

- `main.rs` < 50 lines.
- Each subcommand owns its module + `Run` impl.
- `clap::derive` with explicit `#[command(...)]` attributes.
- No raw `Arg::new` calls in `main.rs`.

## Out of scope

- Refactoring auth flow itself (epic-02).
- Test scaffolding (epic-05).
