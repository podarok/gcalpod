# P0 — Prelude: env-var + custom-file OAuth resolution

Status: **done** (pre-queue work, retroactively captured).

## Goal

Replace silent fallback to upstream-shared OAuth project with an
explicit resolution chain:

1. `GCAL_CLIENT_ID` + `GCAL_CLIENT_SECRET` env vars.
2. `GCAL_SECRET_FILE` env var pointing to JSON path.
3. `~/.gcal/secret.json` (legacy default).
4. Built-in shared OAuth project (warns on stderr).

`GCAL_VERBOSE=1` prints which source was used.

## Files

- `src/util/calendar.rs` — `resolve_secret()` + `SecretSource` enum.
- `README.md` — resolution-order section.
- `docs/custom_auth.md` — env-var setup steps.
- `.gitignore` — ignore `do_not_commit_to_repo/` + `*.secret.json`.

## Steps

1. Add `resolve_secret()` returning `(ApplicationSecret, SecretSource)`.
2. Inline `build_secret()` helper for env-built ApplicationSecret.
3. Refactor `auth()` to consume `resolve_secret()` output + log path.
4. Update README + custom_auth.md with new resolution order.
5. Add scratch dir + secret-file globs to `.gitignore`.

## Tests

- `cargo build --release` clean.
- `cargo clippy` clean on changed file.
- Manual: `GCAL_VERBOSE=1 gcal --help` → exits 0 (no auth call yet).
- Manual: `~/.gcal/secret.json` present → verbose log shows that path.
- Manual: env vars set → verbose log shows env path.

## Validation

- 3 commits on `feat/custom-oauth-config` branch:
  - `feat(auth): add env-var + custom-file OAuth resolution`
  - `chore(gitignore): exclude do_not_commit_to_repo/ + *.secret.json`
- Pushed to fork: `podarok/google-calendar-cli`.
- Upstream PR not yet opened (per owner rule: avoid pushing to base
  repo at this point).

## Out of scope (handed to follow-up phases)

- Per-profile dir layout (P1).
- `auth login/status/logout/switch` subcommands (P2-P4).
- Token expiry display (P3).

## Result

Implemented and pushed to `podarok/google-calendar-cli` on branch
`feat/custom-oauth-config`. Built artifact installed via
`cargo install --path . --locked` at `~/.cargo/bin/gcal`.

Source observation: upstream `auth()` swallowed any read error
silently, falling back to shared client without warning. Replaced
with explicit chain + stderr warning. `GCAL_VERBOSE` added for
debuggability after first-run user confusion about which credentials
were active.

References:
- branch: `feat/custom-oauth-config`
- commits: `61da3d0` (env resolution) + gitignore follow-up
- file ranges: `src/util/calendar.rs:1-104`, `README.md:55-80`,
  `docs/custom_auth.md:35-66`, `.gitignore:17-24`
