# Wave W1 — Full CLI surface

Polish gcal to feature parity with `gcalcli` core surface:
interactive `init` wizard, `config` get/set, `agenda`, `search`,
`edit`, `delete`, `import`, `quick`, conference flag.

**Blocked by W0.** **Blocks W2.**

## Phases

| Phase | Goal | Status |
|---|---|---|
| [P0-init-wizard](P0-init-wizard/) | `gcal init` interactive: walk through Google Console URL prompts, save secret, run first login. | pending |
| [P1-config-cmds](P1-config-cmds/) | `gcal config get/set/path` to read/write `~/.gcal/config.toml`. | pending |
| [P2-agenda-search](P2-agenda-search/) | `gcal agenda [--from --to]` flat list + `gcal search <query> [--from --to]`. | pending |
| [P3-edit-delete](P3-edit-delete/) | `gcal edit <event-id>` (interactive editor) + `gcal delete <event-id>` (with `--yes`). | pending |
| [P4-import-ics](P4-import-ics/) | `gcal import <ics-path> [--calendar]` — parse VCAL/ICS + bulk insert. | pending |
| [P5-quick-conference](P5-quick-conference/) | Polish `quick "<text>"` + ensure `--conference` works on `add` AND `quick`. | pending |

## Done when

- Surface aligned with [gcalcli](https://github.com/insanum/gcalcli)
  core commands (init, list, edit, delete, agenda, search, quick,
  add, import) plus `auth` group from W0.
- `gcal init` covers happy path for first-time user without
  reading external docs.

## Decisions

See [`DECISIONS.md`](DECISIONS.md).

## Out of scope

- `remind` (W2).
- Conky / template formatters (W2).
- Man page (W2).
