# W1-P0 — `gcal init`

## Goal

Interactive first-run wizard. Walks the user through Google Console
project creation, OAuth client download, secret placement, then runs
`auth login`. Reduces friction over reading `docs/custom_auth.md`.

## Surface

```
gcal init [--profile <name>]
```

## Flow

1. Print intro: explain shared-fallback caveat, recommend own project.
2. Open `https://console.cloud.google.com/projectcreate` (via
   `webbrowser` crate). Wait for "press enter when project ready".
3. Open `https://console.cloud.google.com/apis/library/calendar-json.googleapis.com`.
4. Walk OAuth consent screen.
5. Walk credential creation, download JSON.
6. Prompt: "paste path to downloaded JSON" (drag-and-drop friendly).
7. Move JSON to `~/.gcal/profiles/<name>/secret.json` (validate JSON shape first).
8. Call `auth login --profile <name>`.

## Files

- `src/commands/init.rs` (new).
- `Cargo.toml` — add `dialoguer` for prompts, `webbrowser` for URL open.

## Tests

- Unit: JSON validation rejects malformed file with helpful message.
- Snapshot: prompt text.

## Validation

```sh
rm -rf ~/.gcal
gcal init --profile demo
# Walks through; ends with `gcal auth status` showing demo authenticated
```

## Out of scope

- Skipping browser for headless — covered by `auth login --no-browser`
  (W0-P2). `init` assumes desktop.

## Result

_Filled when phase closes._
