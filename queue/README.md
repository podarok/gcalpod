# Queue

Hierarchical work plan for `gcal` (Google Calendar CLI). Pattern
adapted from
[`gogs-cli-queue`](https://github.com/ITCare-Company/template_for_agents/tree/main/process-knowledge-base/gogs-cli-queue).

Each epic = directory. Each task = subdirectory with `README.md`
containing the plan. Each wave gates the next.

## Active epics

| Epic | Goal | Status |
|---|---|---|
| [epic-01-cli-restructure](epic-01-cli-restructure/) | Move `clap` from a single `Command` chain to subcommand tree (`auth`, `config`, `calendars`, `events`). | pending |
| [epic-02-profile-storage](epic-02-profile-storage/) | Multi-profile token + secret storage at `~/.gcal/profiles/<name>/`. Legacy paths still load as `default`. | pending |
| [epic-03-token-refresh](epic-03-token-refresh/) | Detect expired tokens, refresh in-place, surface refresh errors as actionable messages. | pending |
| [epic-04-output-format](epic-04-output-format/) | `--json` global flag + `--no-color` + table/agenda format split. | pending |
| [epic-05-tests](epic-05-tests/) | Snapshot tests for `clap` help output + integration harness with mocked `CalendarHub`. | pending |
| [epic-06-docs](epic-06-docs/) | README + `docs/custom_auth.md` + man page generation via `clap_mangen`. | pending |
| [epic-07-detach-rename](epic-07-detach-rename/) | Detach GitHub fork from `rust-dd/google-calendar-cli`. Rename per Apache 2.0 + GitHub rules. | pending |

## Active waves

| Wave | Summary | Spec |
|---|---|---|
| **W0-mvp-auth-list** | Smallest surface that pulls events for stm32 planning: `auth login/status/logout/switch`, `list --from --to --json`, `calendars list`. **Blocks W1.** | [`W0-mvp-auth-list/`](W0-mvp-auth-list/) |
| W1-full-cli-surface | Polish: `init` wizard, `config get/set`, `agenda`, `search`, `edit`, `delete`, `import`, `quick`, conference flag. | [`W1-full-cli-surface/`](W1-full-cli-surface/) |
| W2-polish | `remind`, formatter modes (conky / template), `clap_mangen` man page. | [`W2-polish/`](W2-polish/) |
| W3-detach-rename | Detach fork + rename project + Apache 2.0 attribution. **Independent of W0/W1/W2** — can run in parallel. | [`W3-detach-rename/`](W3-detach-rename/) |

## Conventions

- Plan format: `Goal / Files / Steps / Tests / Validation / Out of scope`.
- One task = one commit. PRs land on `main` of `podarok/google-calendar-cli`
  fork. Upstream PRs to `rust-dd/google-calendar-cli` only after owner ok.
- Mark task done by appending `## Result` section to its `README.md`
  and committing it together with the code.
- Pre-flight: `cargo fmt --all -- --check`, `cargo clippy --all-targets
  -- -D warnings`, `cargo test --all`.

## House rules

- Latest stable Rust, no MSRV pin.
- Binary name `gcal`. Crate name `gcal`. Repos:
  - upstream: `rust-dd/google-calendar-cli`
  - fork: `podarok/google-calendar-cli` (active development)
- No AI-credit trailers (NDA).
- Owner gates each meaningful action — see [`RULES.md`](RULES.md).
- Backwards compatibility: legacy `~/.gcal/secret.json` and
  `~/.gcal/store.json` continue to work as `default` profile.

## Pinned references

| anchor | role |
|---|---|
| `gogs-cli-queue` | anatomy source |
| `gh auth` | multi-account UX reference |
| `gcalcli` (insanum) | Google Calendar CLI surface reference |
| Google OAuth 2.0 best practices | scope + token-storage guidance |
