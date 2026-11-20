# Command line

```
regolith-doctor [GLOBAL OPTIONS] <COMMAND> [ARGS]
```

Commands marked *planned* parse their arguments but exit with status 4 and the message "not implemented yet".

## Global options

| Option | Meaning |
|---|---|
| `--root <DIR>` | Inspect the system rooted at DIR (default `/`). Every file is read relative to it. |
| `--home <DIR>` | The user's home directory, as a path inside the root. |
| `--user <NAME>` | The user whose configuration is inspected; the home directory comes from the root's passwd file (planned). |
| `--snapshot <DIR>` | Replay command and IPC output recorded with `snapshot save` (planned). |
| `--json` | Machine-readable output. |
| `--no-color` | No colour in text output. |

Global options may come before or after the command.

## Commands

| Command | Does | State |
|---|---|---|
| `check [--area A]... [--id ID]... [--session i3\|sway\|both]` | Run checks and print findings. | works; one check (ENV-001) so far |
| `explain <ID or topic>` | Print a check's documentation. Topics ("how do I change the bar font") are planned. | works for check ids |
| `keys [--session S] [KEY]` | Effective Xresources values per session, with the file and line that set each one. | planned |
| `config [--session S]` | Effective window manager partials with their owning packages, and every binding with its origin. | planned |
| `fix [--dry-run] [--id ID]... [--yes]` | Apply fixes (see [architecture.md](architecture.md#fixes)). | planned |
| `undo [TXID]` | Undo a fix transaction, the latest by default. | planned |
| `history` | List fix transactions. | planned |
| `validate [--session S]` | Run `i3 -C` and `sway -C -d` on the configuration and map every message to a file and line. | planned |
| `snapshot save <DIR>` | Record command and IPC output for replay with `--snapshot`. | planned |
| `report [--redact] [-o FILE]` | Support bundle for bug reports. | planned |

`check --id` with a catalogued but not yet implemented id exits with status 4; an id that is not in the catalog exits with status 3.

## Exit status

| Status | Meaning |
|---|---|
| 0 | No finding at warning level or above. |
| 1 | At least one warning, no errors. |
| 2 | At least one error. |
| 3 | Usage error, internal error, or a check that failed to run. |
| 4 | The command or option is not implemented yet. |

## JSON

`check --json` prints one object:

```json
{
  "schema_version": 1,
  "checks": ["ENV-001"],
  "findings": [
    {
      "id": "ENV-001",
      "severity": "error",
      "sessions": ["i3", "sway"],
      "message": "Arch Linux is not Ubuntu or Debian and not derived from them",
      "evidence": [{ "path": "/etc/os-release", "note": "ID=arch" }],
      "fixes": [{ "class": "report-only", "summary": "Regolith publishes packages only for ubuntu and debian; use one of those or a derivative of them." }]
    }
  ]
}
```

- `checks` lists the checks that ran, sorted.
- `findings` are sorted by id, then most severe first, then by the first evidence path and line, then by message.
- Evidence fields that are unknown are left out. Paths are as seen inside the root.
- `failures` (a list of `{ "id", "message" }`) appears only when a check failed to run.
- `schema_version` changes whenever the format changes incompatibly.

`explain --json` prints `{ "id", "title", "implemented", "text" }`.
