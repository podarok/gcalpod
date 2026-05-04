# P4 — `gcal auth logout` + `gcal auth switch`

## Goal

Bookend the auth lifecycle. `logout` deletes a profile's token (and
optionally secret + dir). `switch` changes the active profile in
`config.toml`.

## Surface

```
gcal auth logout [--profile <name>] [--all] [--purge]
gcal auth switch <profile>
```

Flags:
- `--profile <name>` — target profile. Default: active.
- `--all` — log out every profile.
- `--purge` — also delete `secret.json` and the profile dir.
- `<profile>` (switch) — required positional.

## Behaviour

- `logout` (default): delete `store.json` of the profile.
  Prints "logged out of profile X". If profile was active and other
  profiles exist, prompt to switch (or `--profile` next active).
- `logout --purge`: also revoke token via Google
  (`/oauth2/revoke?token=…`) + remove `secret.json` + remove profile dir.
- `switch`: validate target exists (has `secret.json` or token);
  update `config.toml.active_profile`; print confirmation.

## Files

- `src/commands/auth/logout.rs` (new).
- `src/commands/auth/switch.rs` (new).
- `src/commands/auth/mod.rs` — register both.
- `src/util/oauth_revoke.rs` (new) — POST to
  `https://oauth2.googleapis.com/revoke?token=<token>`.

## Steps

1. Implement `logout` happy path: token delete only.
2. On `--purge`, call revoke endpoint with current token; ignore 200
   vs 400 (token already invalid is fine).
3. Implement `switch`: read + validate + write config atomically.
4. Update prompt UX: when active profile loses auth, suggest next.

## Tests

- Unit: revoke URL shape.
- Unit: `switch` rejects unknown profile with helpful error.
- Integration: `logout` removes only `store.json`; `--purge`
  removes everything.

## Validation

```sh
gcal auth login --profile p1
gcal auth login --profile p2
gcal auth switch p1
gcal auth status  # shows p1 active
gcal auth logout --profile p2
gcal auth logout --all --purge   # nukes everything
```

## Out of scope

- `auth refresh` (force token refresh) — W1 follow-up.
- `auth token` (print bearer) — fold into `status --show-token`.

## Result

_Filled when phase closes._
