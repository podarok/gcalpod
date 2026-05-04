# W3-P2 — Detach or recreate

## Goal

Sever the GitHub fork relationship from `rust-dd/google-calendar-cli`.
Try detach first; fall back to recreate-and-archive.

## Path A: Self-service detach (preferred)

Available via GitHub web UI:
1. Go to https://github.com/podarok/google-calendar-cli/settings.
2. Scroll to "Danger Zone" → "Leave fork network".
3. Confirm. URL stays the same. History intact. `isFork: false`
   after.

Verification:
```sh
gh repo view podarok/google-calendar-cli --json isFork,parent
# expect: {"isFork":false,"parent":null}
```

If GitHub blocks (rare; may happen on private forks of public repos),
go to Path B.

## Path B: Recreate as standalone

```sh
NEW_NAME="<from W3-P1>"
gh repo create podarok/$NEW_NAME --private --description "Google Calendar CLI"
git remote add new git@github.com:podarok/$NEW_NAME.git
git push new --all
git push new --tags
gh repo archive podarok/google-calendar-cli   # mark old read-only
```

Trade-off: separate URL. Open issues / PRs on old repo do not migrate.
Only do this if Path A fails.

## Steps

1. Verify current fork status: `gh repo view podarok/google-calendar-cli
   --json isFork,parent` — expect `isFork:true`.
2. Attempt Path A via web UI. Owner action (cannot self-execute via
   `gh` API).
3. If detached, run verification.
4. If blocked, owner decision: proceed with Path B?

## Files

No file changes in this phase — repo metadata only.

## Tests

None (config change).

## Validation

```sh
gh repo view podarok/<name> --json isFork,parent
# isFork: false, parent: null
```

## Out of scope

- Mirror sync to upstream after detach (no longer applicable).
- Issue migration if Path B taken — separate decision.

## Result

_Filled after detach completes._
