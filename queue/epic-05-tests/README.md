# Epic 05 — Test infrastructure

Snapshot tests for `clap` help output + integration harness with
mocked `CalendarHub`. Required by every wave.

## Scope

- `tests/cli_help.rs` — `assert_cmd` + `insta` snapshots for every
  subcommand's `--help`. Catches regressions in flag names or order.
- `tests/integration/auth.rs` — fake OAuth server (httpmock) +
  golden-path assertions on `auth login` / `status` / `logout`.
- `tests/integration/list.rs` — fixture Calendar API responses,
  verify `list` formats (table / json / tsv / csv / raw).
- `tests/fixtures/events_*.json` — golden Calendar API payloads.
- `tests/integration/profile.rs` — tempdir-backed `HOME`, verify
  profile dir migration.
- CI: `cargo test --all` gate already in `.github/workflows/rust.yml`.

## Done when

- `cargo test --all` runs in <30s on cold cache.
- Every subcommand has a `--help` snapshot.
- Profile + auth + list each have at least one integration test.
- Mocked OAuth flow covers happy path + revoked-token error.

## Out of scope

- E2E tests against real Google API (manual smoke only — too flaky
  for CI).
