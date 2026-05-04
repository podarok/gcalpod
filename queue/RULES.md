# Queue Rules

Execution constraints for every epic + wave + phase in `queue/`.
Adapted from
[`gogs-cli-queue/RULES.md`](https://github.com/ITCare-Company/template_for_agents/blob/main/process-knowledge-base/gogs-cli-queue/RULES.md).

**Aligned with gogs-cli-queue**: **no PRs.** Commits land directly
on `main`. One commit per feature. Push after owner ok.

(Earlier draft required PRs because the repo was a fork. After the
W3 detach + rename — now standalone `podarok/gcalpod` — the project
moved to the gogs-cli-queue model: commit per feature on `main`,
no PR overhead.)

## Dialogue gates

1. **Plan-first.** Before code, post the `## Plan` of the task in
   chat. Wait for explicit "ok" / "go" / "вйо" before touching
   `src/`.
2. **Verify-first.** After pre-flight green, run the manual verify
   snippet (e.g. `gcal auth login --profile demo`). Post output.
   Wait for "ok" / "push".
3. **Destructive-first.** Force-push, branch delete, history
   rewrite, dependency removal — separate confirmation each time.
4. **Memory-first.** Owner rule ("never X", "always Y") goes into
   [`README.md`](README.md) → "House rules" in the same turn.

## Commit shape

- One task = one commit. Commits land directly on `main` of
  `podarok/gcalpod`. **No PRs.** Push after owner "ok" / "вйо".
- Optional short-lived feature branches for isolation are fine —
  fast-forward into `main` (no merge commits, no squash) and
  delete the branch afterwards.
- Subject under 70 characters. Conventional Commits prefix:
  - `feat(auth): …` for `src/auth/*`
  - `feat(cli): …` / `fix(cli): …` for command surface
  - `feat(config): …` for `src/config/*`
  - `feat(events): …` for event commands (list/agenda/search/edit/...)
  - `feat(calendars): …` for calendar listing / selection
  - `chore(queue): …` for queue scaffolding / index updates
  - `docs(queue): …` / `docs(arch): …` for docs
  - `test(...)` / `refactor(...)` per area
- Body explains **why**, references plan path
  (`see queue/W0-mvp-auth-list/P1-auth-login/`).
- **No** AI-credit trailers (NDA). **No** "Generated with Claude Code"
  footers.
- Never amend a published commit.

## Pre-flight gates (every task)

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
cargo run --release -- --help    # smoke-check binary
```

All must be clean before commit.

## Queue maintenance

If a completed task surfaces a nuance the queue did not anticipate
(rejected design, missing dependency, tool gotcha, redesigned
subcommand), the **next** commit before further code work updates
the queue:

- Update the affected plan's `## Result` and downstream plans'
  `## Goal` / `## Steps` / `## Out of scope`.
- Update [`INDEX.md`](INDEX.md), wave `DECISIONS.md` — whichever
  holds the contradicted detail.
- Note source observation (`see commit <sha>` or PR #).

`chore(queue): …` commits stay tiny — one nuance per commit.

## Tests must not hardcode personal identifiers

Tests must not embed real Google account emails, real calendar IDs,
or real Client IDs. Use anonymous fixtures (`alice@example.com`,
`primary`, `test-client-id`) or constants pulled from a single
fixtures module.

**Why:** Hardcoded names drift and leak attribution into commit history.

**How to apply:** Replace any `podarokua@gmail.com` / real
`*.apps.googleusercontent.com` IDs with fixture constants. Real
secrets stay only in `~/.gcal/` (never in repo).

## Secret hygiene

- OAuth client secrets never enter git. `do_not_commit_to_repo/`
  + `*.secret.json` ignored via `.gitignore`.
- Token store (`store.json`) never in repo.
- Real client IDs in `src/util/calendar.rs` (FALLBACK_*) are the
  shared/upstream demo project — owner of those credentials is
  upstream `rust-dd`. Do not extend with new hardcoded credentials.

## Backwards compatibility

- `~/.gcal/secret.json` continues to work as `default` profile.
- `~/.gcal/store.json` continues to work as `default` profile token.
- Migration to `~/.gcal/profiles/default/` happens on first
  `gcal auth login --profile default` (move, not copy).
- Env vars `GCAL_CLIENT_ID` / `GCAL_CLIENT_SECRET` /
  `GCAL_SECRET_FILE` / `GCAL_VERBOSE` continue to apply across
  all profiles.

## Default branch policy

- All work lands on `main` of `podarok/gcalpod` (standalone, no
  longer a fork after W3 detach + rename).
- No mirror back to `rust-dd/google-calendar-cli` (upstream relation
  severed). Attribution preserved via `LICENSE` + `NOTICE.md` only.
