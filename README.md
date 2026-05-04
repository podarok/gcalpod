<div align="center">

# gcalpod

**Google Calendar CLI in Rust — multi-profile auth, range queries,
JSON/TSV/CSV/raw/conky output, ICS import, hybrid grid/agenda rendering.**

[![rust](https://github.com/podarok/gcalpod/actions/workflows/rust.yml/badge.svg)](https://github.com/podarok/gcalpod/actions/workflows/rust.yml)
[![Sponsor](https://img.shields.io/badge/Sponsor-podarok-ff69b4)](https://github.com/sponsors/podarok)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

</div>

> [!NOTE]
> Binary is `gcal`. Crate / repo name is `gcalpod`. Derived from
> [`rust-dd/google-calendar-cli`](https://github.com/rust-dd/google-calendar-cli)
> (Apache 2.0). Substantial modifications listed in
> [`NOTICE.md`](NOTICE.md).

---

## Highlights

- 🔐 **Multi-profile OAuth** — `~/.gcal/profiles/<name>/` per account; share
  one OAuth client across profiles via `gcal init --shared`.
- 📅 **Range queries** with natural-language input — `today`, `+7d`,
  `+2w`, weekday names, RFC3339, `YYYY-MM-DD`.
- 🧾 **Six output formats** — `table` / `json` / `tsv` / `csv` / `raw` /
  `conky`. Auto-pretty JSON on tty, compact when piped.
- 🪟 **Hybrid renderer** — week grid for short ranges on wide
  terminals, day-grouped agenda for long ranges or narrow terms.
- 📥 **ICS import** with `--dry-run` and `--skip-duplicates`.
- ⏰ **`gcal remind`** — exec a command N minutes before next event
  with `{{summary}}` / `{{start}}` / `{{html_link}}` interpolation.
- 🧯 **Failure-first errors** — RTK-style one-line summary +
  recovery-metadata path to the full log.
- 🧭 **`--verbose`** for newcomers / init agents.
- 📜 **Man page** via `gcal --gen-man`.

## Install

```sh
git clone git@github.com:podarok/gcalpod.git
cd gcalpod
cargo build --release && cargo install --path . --locked
```

The binary lands at `~/.cargo/bin/gcal`.

> [!IMPORTANT]
> `gcal` requires your own Google Cloud OAuth client. There is **no
> shared / built-in fallback** — every user creates their own OAuth
> project (~5 minutes, free for personal use).

## Quickstart

```sh
gcal init                     # interactive wizard
gcal auth status              # confirm authentication
gcal list                     # current week (table)
```

Or, for multi-account workflows:

```sh
gcal init --shared            # one OAuth client at ~/.gcal/secret.json
gcal auth login --profile work
gcal auth login --profile personal
gcal auth switch work
gcal list --from today --to +7d
```

<details>
<summary><b>Configuring your Google Cloud OAuth client manually</b></summary>

1. Visit https://console.cloud.google.com/projectcreate and create a project.
2. Enable Calendar API at https://console.cloud.google.com/apis/library/calendar-json.googleapis.com
3. Set up the OAuth consent screen (External / Testing) — add your email as a test user.
4. Create OAuth client ID (Application type: **Desktop app**) and download the JSON.
5. Move it into place:
    ```sh
    mkdir -p ~/.gcal
    mv ~/Downloads/client_secret_*.json ~/.gcal/secret.json
    ```

Detailed walkthrough: [`docs/custom_auth.md`](docs/custom_auth.md).

</details>

## Commands

<table>
  <thead>
    <tr><th>Command</th><th>Purpose</th></tr>
  </thead>
  <tbody>
    <tr><td><code>gcal init [--shared]</code></td><td>Interactive setup wizard (per-profile or shared secret).</td></tr>
    <tr><td><code>gcal auth login</code></td><td>OAuth flow for a profile. Flags: <code>--scopes</code>, <code>--no-browser</code>, <code>--reauth</code>.</td></tr>
    <tr><td><code>gcal auth status [--all] [--check]</code></td><td>Per-profile state. <code>--check</code> pings live API.</td></tr>
    <tr><td><code>gcal auth switch &lt;profile&gt;</code></td><td>Change active profile in <code>~/.gcal/config.toml</code>.</td></tr>
    <tr><td><code>gcal auth logout [--all] [--purge]</code></td><td>Remove cached token (and secret with <code>--purge</code>).</td></tr>
    <tr><td><code>gcal auth refresh</code></td><td>Force token refresh.</td></tr>
    <tr><td><code>gcal list [--from --to --calendar --format --style --lineart]</code></td><td>Range query. Hybrid grid/agenda by default.</td></tr>
    <tr><td><code>gcal agenda</code> / <code>gcal search &lt;q&gt;</code></td><td>Flat list / full-text search.</td></tr>
    <tr><td><code>gcal calendars list</code></td><td>List accessible calendars.</td></tr>
    <tr><td><code>gcal add</code> / <code>gcal quick &lt;text&gt;</code></td><td>Create events; <code>--conference</code> attaches Google Meet.</td></tr>
    <tr><td><code>gcal edit &lt;id&gt; --field key=value</code></td><td>Mutate fields: summary, description, location, start, end.</td></tr>
    <tr><td><code>gcal delete &lt;id&gt; [-y]</code></td><td>Delete with confirmation gate.</td></tr>
    <tr><td><code>gcal import &lt;path&gt; [--dry-run] [--skip-duplicates]</code></td><td>Bulk-insert ICS / VCAL.</td></tr>
    <tr><td><code>gcal remind &lt;mins&gt; -- &lt;cmd&gt;...</code></td><td>Exec command N min before next event.</td></tr>
    <tr><td><code>gcal config get/set/unset/list/path</code></td><td>Manage <code>~/.gcal/config.toml</code>.</td></tr>
    <tr><td><code>gcal --gen-man</code></td><td>Print man page (clap_mangen) to stdout.</td></tr>
  </tbody>
</table>

Add `--verbose` to any command for extra context — useful for new
users and AI init agents.

## Output formats

```sh
gcal list                                           # table (default)
gcal list --format json | jq '.[] | select(.summary | test("standup"; "i"))'
gcal list --from today --to +30d --format tsv > planning.tsv
gcal list --format csv | column -t -s,
gcal list --format raw | jq '.[] | .recurrence'    # full upstream Event JSON
gcal list --format conky                            # ${color cyan}…${color}
```

Stable v1 schema for `--format json|tsv|csv`:

```json
{
  "id": "...",
  "calendar_id": "primary",
  "summary": "...",
  "description": "...",
  "start": "2026-05-04T12:00:00+03:00",
  "end":   "2026-05-04T13:00:00+03:00",
  "all_day": false,
  "status": "confirmed",
  "creator": "...",
  "attendees_count": 2,
  "html_link": "...",
  "updated": "..."
}
```

Bumping any field name / removing one is a breaking change → major version.

## Configuration

`gcal` resolves OAuth credentials in this order:

1. `GCAL_CLIENT_ID` + `GCAL_CLIENT_SECRET` env vars (optional `GCAL_PROJECT_ID`).
2. `GCAL_SECRET_FILE=<path>` env var.
3. `~/.gcal/profiles/<active>/secret.json`.
4. `~/.gcal/secret.json` (shared / legacy — one client across profiles).

Active-profile resolution: `--profile` flag → `GCAL_PROFILE` env →
`~/.gcal/config.toml` `active_profile` → `"default"`.

```sh
gcal config set tz Europe/Kyiv
gcal config set default_format json
gcal config list
gcal config path
```

`GCAL_VERBOSE=1` (or `--verbose`) prints which source supplied the
secret + profile in use.

## Failure-first errors

```text
$ gcal auth switch nonexistent
gcal: auth switch failed: Profile 'nonexistent' does not exist. Create
it via `gcal auth login --profile nonexistent` first.; see /Users/.../
Library/Application Support/gcal/tee/1777882749_auth_switch.log
```

Pass `-v` / `--verbose` to print the full error inline (no log path).

## Development

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all                 # 47+ tests
cargo run --release -- --help
```

> [!TIP]
> `gcal --gen-man > /usr/local/share/man/man1/gcal.1` then `man gcal`.

## Sponsorship

Sponsor button on the repo page activates these channels (configured
in [`.github/FUNDING.yml`](.github/FUNDING.yml)):

- [GitHub Sponsors](https://github.com/sponsors/podarok)
- [Patreon](https://www.patreon.com/podarok_ua)
- [Buy Me a Coffee](https://www.buymeacoffee.com/podarok)
- [PayPal](https://www.paypal.com/ncp/payment/HW9T9M6U8ZGVU)

## Working method

Starting **v1.0.0**, all changes to `main` flow through **open
issues + pull requests** (`PR/Issue gate`). Direct pushes to `main`
are blocked. See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the full
workflow:

1. Open an issue describing intent.
2. Wait for maintainer ack.
3. Fork or branch → push commits.
4. Open a PR; CI must pass (`fmt`, `clippy`, `test`, smoke).
5. Maintainer review + merge.

The full v1.0.0 work plan and decision history were transferred
upstream to [`ITCare-Company/template_for_agents/process-knowledge-base/gcalpod-queue/`](https://github.com/ITCare-Company/template_for_agents/tree/main/process-knowledge-base/gcalpod-queue)
as a worked Anatomy reference.

## Acknowledgements

Built on top of [`rust-dd/google-calendar-cli`](https://github.com/rust-dd/google-calendar-cli).
Substantial modifications + fork-point commit hash: [`NOTICE.md`](NOTICE.md).

## License

This project is licensed under the [PolyForm Noncommercial License 1.0.0](LICENSE)
**plus** the [gcalpod Sustainable License Addendum v1 (gSL-v1)](LICENSE-ADDENDUM.md).

Quick guide:

| If you are... | What applies | What you owe |
|---|---|---|
| Hobbyist / student / employee on personal time | core license (Noncommercial) | nothing |
| Solo or 2-person micro-business, < $20k revenue, ≤ $20k raised | Addendum B | self-assess; nothing |
| Contributor with merged commits | Addendum E | nothing; commercial use granted |
| Larger company sponsoring ≥ $5/mo on GH Sponsors / Patreon / BMC | Addendum A | maintain sponsorship |
| Larger company without sponsorship | core license (Noncommercial only) | sponsor, license, or wait 4y (Addendum C) |
| Distro / registry packager | Addendum D | preserve LICENSE files |

Each tagged release auto-converts to [Apache 2.0](LICENSE-Apache-2.0)
four years after its tag date (Addendum C — anti-lock-in).

Upstream-derived portions remain available under [Apache License 2.0](LICENSE-Apache-2.0)
per Apache 2.0 §4. See [`NOTICE.md`](NOTICE.md) for full attribution.
