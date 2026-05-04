# W5-P2 — Implement

## Goal

Apply the W5-P1 owner-picked option across all `gcal` subcommands.

## Steps (option-agnostic skeleton)

1. Add global flag (`-u/--ultra-compact` and/or `-v/--verbose`) in
   `src/main.rs` (clap derive).
2. Add `OutputMode { Verbose, Compact }` to `src/util/format.rs`.
3. Per-command renderer branches on `OutputMode` for human output.
   `--format json|tsv|csv|raw` paths unchanged (already
   token-optimized).
4. Trim `--help` text:
   - Single-line flag descriptions.
   - Drop default-value tags where redundant.
   - Drop `[possible values]` lists when format is documented elsewhere.
5. Failure-first: route every error through a unified printer that
   emits `<error msg>; see <log path>` with full output saved to
   `~/.local/share/gcal/tee/<unix>_<cmd>.log`.
6. Add tests:
   - Snapshot: `--help` byte length stays under target (e.g. 80
     chars/flag-line, total help < 1.5KB per command).
   - Failure path: error → full log written + path printed.

## Files

- `src/util/format.rs` — `OutputMode`, recovery-tee writer.
- `src/main.rs` — global flag wiring.
- Per-command modules — renderer branches.
- Snapshot tests under `tests/cli_help.rs`.

## Tests

- Help-length snapshots (catch regression in noise).
- Failure-tee writes full log with read-back assertion.

## Validation

```sh
gcal list --help        # under target byte budget
gcal list --from today  # 1-3 line compact output (if Option B/C)
gcal list -u            # ultra mode (if Option A/C)
gcal list bogus-flag    # error printed compact + tee path
```

## Out of scope

- Wrapping `gcal` invocation inside `rtk` proxy.
- Conky / Tera template formatters (W2-P1).

## Result (in flight)

**Step 1 done (2026-05-04):** failure-first + recovery tee.

Files:
- `src/util/recovery.rs` (new) — `report_error(command, error)`
  writes full error body to `~/Library/Application Support/gcal/tee/<unix>_<cmd>.log`
  (XDG `data_local_dir`/gcal/tee on Linux; `/tmp/gcal-tee` fallback)
  and prints `gcal: <cmd> failed: <first-line-truncated-120>; see <path>`.
  3 unit tests for `truncate_first_line` cover short-pass, long-clip,
  multi-line-first-line semantics.
- `src/util/mod.rs` — register `recovery`.
- `src/main.rs` — replace 9 operational error sites with
  `util::recovery::report_error("<cmd>", &e)`: init, config, auth
  login/status/logout/switch, authentication build, remind, import,
  edit, delete. Argument-parse errors (e.g. bad `--from`) still use
  direct `eprintln!` because they're already concise + recoverable
  by the user.

Smoke verified:
```
$ gcal auth switch nonexistent
gcal: auth switch failed: Profile 'nonexistent' does not exist. ... ; see /Users/.../gcal/tee/1777882749_auth_switch.log
```
Log file present + holds full error body.

**Step 2 pending:** `--help` text trim across all subcommands.
**Step 3 pending:** `-u/--ultra-compact` global flag + per-command
compact renderers.
