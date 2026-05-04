# W3-P4 — Cargo rename

## Goal

Rename Cargo package + binary. Update install path.

## Files

- `Cargo.toml` — `package.name`, `package.description`,
  `package.repository`, `package.homepage`.
- `src/main.rs` — `Command::new("<old>")` → new name (clap).
  Note: command surface `gcal ...` may stay if `gcal` remains binary
  alias.

## Steps

1. Decide: does binary name match new repo name? Recommended yes for
   discoverability.
   - If binary stays `gcal`: only `Cargo.toml` `name` changes; users
     keep typing `gcal`.
   - If binary changes too: rename binary in `Cargo.toml`
     `[[bin]]` section + clap `Command::new`.
2. Update `repository = "https://github.com/podarok/<new-name>"`.
3. Update `Cargo.lock` via `cargo build`.
4. Update `cargo install --path .` invocations in docs.

## Tests

- `cargo build --release` clean.
- `cargo install --path . --locked` succeeds; `~/.cargo/bin/<binary>`
  appears.
- `<binary> --version` reports new package name.

## Validation

```sh
cargo build --release
cargo install --path . --locked
gcal --version    # or new binary name
```

## Out of scope

- crates.io publishing (requires owner decision).
- Homebrew formula update (separate distribution channel).

## Result

_Filled when rename committed._
