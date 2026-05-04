# gcalpod — Google Calendar CLI

![rust](https://github.com/podarok/gcalpod/actions/workflows/rust.yml/badge.svg)
[![Sponsor](https://img.shields.io/badge/Sponsor-podarok-ff69b4)](https://github.com/sponsors/podarok)

`gcalpod` is a Rust command-line interface for Google Calendar. The
binary is invoked as `gcal`; the package + repository name is
`gcalpod`. Add events, list events, manage multiple profiles — all
without leaving your terminal.

> Derivative of [`rust-dd/google-calendar-cli`](https://github.com/rust-dd/google-calendar-cli)
> (Apache 2.0). Detached fork with substantial modifications. See
> [`NOTICE.md`](NOTICE.md).

![Screenshot](docs/screenshot.png)

***

## Status (2026-05-04)

**Shipped on `main`:**
- Custom Google Cloud OAuth client (env vars / file / `~/.gcal/secret.json`).
- `GCAL_VERBOSE=1` source logging.
- `add`, `list` (current week), top-level quick-add.
- Conference (Google Meet) support on event create.
- Apache 2.0 attribution + Sponsor button.

**On the queue (not yet implemented):**
- Multi-profile auth (`gcal auth login/status/logout/switch`).
- `gcal list --from --to --format json|tsv|csv|raw|table`.
- `gcal calendars list`.
- `gcal init` interactive setup wizard.
- `gcal config get/set`, `agenda`, `search`, `edit`, `delete`,
  `import`, `remind`.
- Man page generation.

Roadmap detail: [`queue/INDEX.md`](queue/INDEX.md). Working method:
no PRs, commit per feature on `main`.

## Installation

```sh
git clone git@github.com:podarok/gcalpod.git
cd gcalpod
cargo build --release && cargo install --path . --locked
```

After install, the binary is `gcal` at `~/.cargo/bin/gcal`.

## First-time auth setup

`gcal` requires your own Google Cloud OAuth client. There is **no
shared / built-in fallback** — every user creates their own OAuth
project (~5 minutes, free for personal use).

Step-by-step Google Cloud Console setup: [`docs/custom_auth.md`](docs/custom_auth.md).

## Usage

```sh
gcal help                                        # show subcommand tree
gcal "Retro & Demo at 16:00"                     # quick-add (today)
gcal "Appointment on June 3rd 10am-10:25am"      # quick-add (natural date)
gcal "Appointment" "10:25"                       # quick-add with time
gcal add "Sprint planning" "2026-05-06 10:00"    # explicit add
gcal "Appointment" "23:45" --conference          # add with Google Meet
gcal list                                        # current week table
```

`--from / --to / --format` flags for `list` are on the queue
([W0-P5](queue/W0-mvp-auth-list/P5-list-range/), [W0-P6](queue/W0-mvp-auth-list/P6-list-json/)).

## Configuration

`gcal` resolves OAuth credentials in this order:

1. `GCAL_CLIENT_ID` + `GCAL_CLIENT_SECRET` env vars (optional `GCAL_PROJECT_ID`).
2. `GCAL_SECRET_FILE=<path>` env var.
3. `~/.gcal/secret.json` (default file path).

If none are configured, `gcal` errors with a setup pointer.

```sh
GCAL_VERBOSE=1 gcal list
# gcal: OAuth secret from /Users/you/.gcal/secret.json
```

The OAuth token is cached at `~/.gcal/store.json` after the first
successful login. Delete that file to re-authenticate.

## Development

```sh
cargo run -- list                            # run from source
cargo fmt --all -- --check                   # pre-flight
cargo clippy --all-targets -- -D warnings    # pre-flight
cargo test --all                             # pre-flight
```

Working method: commits land directly on `main`. No PRs. See
[`queue/RULES.md`](queue/RULES.md).

## Sponsorship

Sponsor button on the repo page activates these channels (configured
in [`.github/FUNDING.yml`](.github/FUNDING.yml)):

- [GitHub Sponsors](https://github.com/sponsors/podarok)
- [Patreon](https://www.patreon.com/podarok_ua)
- [Buy Me a Coffee](https://www.buymeacoffee.com/podarok)
- [PayPal](https://www.paypal.com/ncp/payment/HW9T9M6U8ZGVU)

## Acknowledgements

Built on top of `rust-dd/google-calendar-cli`. Substantial
modifications and full attribution: [`NOTICE.md`](NOTICE.md).

## License

[Apache License 2.0](LICENSE) (preserved from upstream). Modifications
released under the same license.
