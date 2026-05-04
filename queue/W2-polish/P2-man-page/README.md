# W2-P2 — Man page generation

## Goal

Ship `man gcal` via `clap_mangen` build-time generation.

## Surface

```
make install        # installs binary + manpage to /usr/local/share/man
gcal --gen-man      # prints man page to stdout (for packagers)
```

## Files

- `build.rs` (new) — generate `target/man/gcal.1` from clap command tree.
- `Makefile` (new) — install binary + manpage.
- `Cargo.toml` — add `clap_mangen` build-dep.

## Tests

- Snapshot: man page TOC stable across builds.

## Validation

```sh
make install
man gcal           # full reference
```

## Out of scope

- Bash / zsh / fish completions — separate feature if requested.
  (`clap_complete` build script can plug in alongside.)

## Result

Implemented 2026-05-04 on `main`. Runtime `--gen-man` instead of
`build.rs` (simpler, no compile-time artifact tracking).

Files:
- `Cargo.toml` — add `clap_mangen` 0.2.
- `src/main.rs` — extract clap tree into `fn build_cli() -> Command`.
  Add `--gen-man` flag at top level; when set, `clap_mangen::Man::new(cmd)`
  renders to stdout + exits before any auth.

Install pattern:
```sh
cargo install --path . --locked
gcal --gen-man > /usr/local/share/man/man1/gcal.1
man gcal
```
