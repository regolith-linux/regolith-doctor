# Architecture

regolith-doctor looks at a Regolith system, reports what is wrong and, where it is safe, fixes it. This page describes the intended design. Parts that exist today are marked **(now)**; the rest is planned.

## Crates

| Crate | Role |
|---|---|
| `crates/doctor-core` | Types every other part shares: check ids, severities, findings, reports, the check trait and registry, and file access under the root **(now)**. Later also the system snapshot, the parsed views of apt, packages, resources and WM config, and the fix engine. |
| `crates/doctor-checks` | The checks, one module per area, and the text of `docs/checks/*.md` for `explain` **(now: ENV-001)**. |
| `crates/regolith-doctor` | The command-line program **(now)**. |
| `crates/regolith-doctor-gtk` | A GTK4 window over the same core (planned). It contains no check logic. |

`data/` holds tables the checks need (for example `data/platforms.toml`). They are compiled into the binary, so the doctor works offline. `docs/checks/<ID>.md` documents every check, implemented or not.

## Reading the system

1. **Everything goes through the root.** Checks never open files directly. They ask `RootFs` for absolute paths as they would appear on the inspected system (`/etc/os-release`), and `RootFs` maps them under `--root` (default `/`) **(now)**. Running against `/` inspects the live machine; running against a directory inspects a fixture tree or a mounted system image. Planned: symbolic links resolved as in a chroot, so an absolute link inside the root stays inside it and nothing outside the root is ever read.
2. **Snapshot.** A `Snapshot` type gathers the facts once per run: files under the root (os-release, dpkg status and file lists, apt sources and lists, `/etc/regolith`, `/usr/share/regolith`, looks, GSettings schemas and overrides, autostart entries, portal files, excerpts of `/sys` and `/proc`) and the user's home directory (`--home`, or the home of `--user` from the root's passwd file). Commands and IPC replies (i3 and sway IPC, D-Bus name owners, the systemd user environment, the journal, `dconf dump`) are *probes*: each has a live backend and a replay backend, so `snapshot save` can record them and `--snapshot` can replay them later.
3. **Installed set first.** Before any configuration check, a shared layer reads `/var/lib/dpkg/status` and `/var/lib/dpkg/info/*.list` and works out which packages are installed and which of their files are really on disk. Regolith does not assume every package is installed: removing a package and dropping in one's own partial is a documented way to replace it. A user file that stands in for a removed package's file is therefore valid; collisions count only with files whose package is installed.
4. **Parsed views, built once:** apt sources, packages and versions, the Xresources resolver (the preprocessor subset the looks use, load order, look selection), the window manager configuration (effective include list for i3 and sway, variables, bindings, bars, execs), GSettings (schema defaults, Regolith overrides per desktop, user values), and environment, desktop entry and portal files.

## Checks

A check implements the `Check` trait **(now)**: it declares its id, title, sessions and fix class, reads facts from the `Context`, and returns findings. It never changes anything. The `Registry` holds all checks sorted by id and runs a selection of them (by id, area or session). A check that fails to run (for example an unreadable file) is recorded as a failure in the report and does not stop the others. Planned: each check also declares the facts it needs, so a run without a live session marks session-only checks as skipped.

Check ids (`AREA-NNN`) are stable. The current areas are APT, BAR, CFG, COMP, DISP, ENV, INPUT, KBD, LNCH, LOOK, NOTIF, PKG, PORT, PWR, SES and XRES.

## Findings and reports

A finding has the check id, a severity (`error`, `warning`, `info`), the sessions it affects (`i3`, `sway`, `gnome`), a message, evidence (path inside the root, line, owning package, probe, note) and fix proposals with their safety class **(now)**. A report lists the checks that ran, the findings in a fixed order and any failures; `--json` prints it with a `schema_version` field. See [cli.md](cli.md) for exit codes.

## Fixes

Each fix has a safety class:

- **auto**: reversible, with backup and undo; inside the user's home, or a harmless live action such as a reload. Root-owned auto fixes only with `--yes` as root.
- **guided**: the doctor shows the exact change or command and applies it only after confirmation.
- **report-only**: the doctor explains and changes nothing.

Planned fix engine: a fix is a plan of file operations (edit with the expected old content hash, create, move aside) or a command for the user to run. `fix --dry-run` prints unified diffs. `fix` applies auto fixes and, after confirmation, guided ones. Every apply is a transaction with backups under `$XDG_STATE_HOME/regolith-doctor/<txid>/` and a journal; `undo` restores files byte for byte, with their modes, and refuses if a file changed since. Applying the same fix twice changes nothing the second time. Files owned by root change only when running as root with `--yes`; otherwise the doctor prints the command.

## Support bundle

`report` collects what Regolith's `regolith-diagnostic` script prints (version, session type, installed sessions, kernel and release, resource files and loaded resources, window manager versions and configs, shipped partials, bar blocks, relevant GSettings), using its own code rather than calling the script, and adds what the script lacks: user partials, apt sources, package versions and the installed set, findings, and redaction of user names, home paths, host names and similar data.

## Testing

Checks are tested against small fixture trees under `tests/fixtures/` (files only, no symbolic links in the repository; tests that need links create them at run time). Expected results are compared as data (id, severity, session, path, line), not as text. Checks that need a live session are tested by replaying recorded probe output.
