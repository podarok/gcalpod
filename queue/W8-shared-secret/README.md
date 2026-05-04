# Wave W8 — Shared OAuth secret across profiles

Owner request 2026-05-04: enable multi-account auth without
re-registering an OAuth client per profile. Single Google Cloud
OAuth client id+secret should authenticate many user accounts
(many profiles), each with its own token store.

**Required for v1.0.0** per owner directive.

## Phases

| Phase | Goal | Status |
|---|---|---|
| [P0-design](P0-design/) | Decide resolution + UX. | done |
| [P1-implement](P1-implement/) | Land code + tests + docs. | pending |

## Background

Current resolution chain:
1. `GCAL_CLIENT_ID` + `GCAL_CLIENT_SECRET` env (in-memory).
2. `GCAL_SECRET_FILE=<path>` env.
3. `~/.gcal/profiles/<active>/secret.json`.
4. `~/.gcal/secret.json` (legacy fallback).

Step 4 already lets one secret.json apply to **every** profile that
lacks a per-profile secret. Need to expose this as a first-class
"shared secret" path and document it.

## Decisions

See [`DECISIONS.md`](DECISIONS.md).

## Done when

- A single OAuth client JSON dropped at `~/.gcal/secret.json` (or
  `~/.gcal/shared/secret.json`) authenticates any number of
  profiles, each with its own `store.json`.
- New `gcal init --shared` mode places the JSON in the shared
  location instead of per-profile.
- `gcal auth login --profile X` automatically uses shared secret if
  per-profile secret is absent.
- Docs explain the workflow.
- Tests cover the shared path.

## Out of scope

- Cross-machine sync of secret.json.
- Encrypted secret storage (e.g. OS keychain) — separate feature.
