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

_Filled when phase closes._
