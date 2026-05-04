# W4 Decisions

Append-only.

---

## 2026-05-04 — Snapshot of completed vs in-flight

### Completed (in `main`)

- Env-var + custom-file OAuth resolution (`GCAL_CLIENT_ID`/
  `GCAL_CLIENT_SECRET`/`GCAL_PROJECT_ID`/`GCAL_SECRET_FILE`).
- `GCAL_VERBOSE=1` source logging.
- `.github/FUNDING.yml` (Option D union: GH Sponsors + Patreon + BMC + PayPal).
- W3 detach + rename: standalone `podarok/gcalpod`.
- `NOTICE.md` Apache 2.0 attribution.
- `Cargo.toml` metadata (name=`gcalpod`, binary=`gcal`, license, repo, keywords).
- Queue scaffold (W0..W3 + 7 epics + INDEX + RULES).
- Hardcoded fallback OAuth secret REMOVED (security fix).
- Git history purged of secret via `git-filter-repo`.
- Single-branch repo (`main` only). No PRs going forward.

### In flight (queue, not yet implemented)

- W0 P1-P7: profile storage, `auth login/status/logout/switch`,
  `list --from --to`, `--format json/tsv/csv/raw`, `calendars list`.
- W1: init wizard, config, agenda, search, edit, delete, import, quick.
- W2: remind, formatters, man page.
- W3 already done at the wave level (rename committed); P0-P5
  retroactively closed.

### Constraints

- README must reflect what works TODAY, not the queue (avoid
  promising unimplemented features).
- Roadmap section points to `queue/INDEX.md` for the rest.
