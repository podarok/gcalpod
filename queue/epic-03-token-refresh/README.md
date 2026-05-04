# Epic 03 — Token refresh

Detect expired tokens, refresh in-place, surface refresh errors as
actionable messages. yup-oauth2 handles refresh under the hood, but
we need to surface failures and offer recovery.

## Scope

- Wrap every `auth.token(scopes).await` call in retry-with-refresh
  logic. On `401 Unauthorized`, force a refresh and retry once.
- On refresh failure (refresh_token revoked, network), exit with
  `gcal auth login --reauth --profile <name>` suggestion.
- Add `gcal auth refresh [--profile]` for explicit refresh
  (matches `gh auth refresh` UX).
- `gcal auth status` shows time until expiry.

## Done when

- 24h+ idle session can run `gcal list` without re-prompting auth.
- Revoked token surfaces clear error + reauth command.
- `gcal auth refresh` succeeds for a valid refresh_token, fails
  cleanly otherwise.

## Out of scope

- Cross-host refresh (only Google APIs covered).
- Service-account auth — defer.
