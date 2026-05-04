<!-- Thanks for a PR. Fill out the sections below. Direct pushes to
`main` are blocked since v1.0.0; this template is the only path. -->

## Summary

<!-- 1-3 bullets describing the change. -->

## Linked issue

Closes # <!-- or Refs # -->

## Type of change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (would cause existing functionality to change)
- [ ] Documentation / tests only

## Test plan

<!-- How did you verify? Include the exact commands. -->

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
# manual smoke:
# gcal ...
```

## Screenshots / output

<!-- For UX changes, paste before/after terminal output. -->

## Checklist

- [ ] Issue opened first; maintainer acked the design.
- [ ] Conventional Commits subject (`feat(area): ...`, `fix(area): ...`).
- [ ] No AI co-author trailers; no `Generated with X` footers.
- [ ] `cargo fmt --all -- --check` clean.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` clean.
- [ ] `cargo test --all` green.
- [ ] CHANGELOG.md updated under `[Unreleased]` if user-facing.
- [ ] No new hardcoded credentials, emails, or personal identifiers
      in source / tests.
- [ ] Upstream attribution preserved (`NOTICE.md`, `LICENSE-Apache-2.0`).

## License acknowledgement

By submitting this PR I agree that my contribution is licensed
under [`LICENSE`](../LICENSE) (PolyForm Noncommercial 1.0.0) plus
[`LICENSE-ADDENDUM.md`](../LICENSE-ADDENDUM.md) (gSL-v1). In return
I receive automatic commercial-use rights via Addendum E so long
as my commits remain in `main`.
