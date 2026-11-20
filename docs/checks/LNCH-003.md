# LNCH-003: ilia GSettings schema missing or not compiled

| | |
|---|---|
| Area | LNCH |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | guided |
| Status | planned |

## What it detects

ilia fails with `Settings schema 'org.regolith-linux.ilia' is not installed`.

## How it is detected

The doctor checks that `/usr/share/glib-2.0/schemas/org.regolith-linux.ilia.gschema.xml` exists and that `gschemas.compiled` exists and is newer than it.

## Fix

The doctor shows the `glib-compile-schemas` command or the package reinstall that fixes it. Guided: the command is shown and run only after you confirm.

## Sources

- https://github.com/regolith-linux/ilia/issues/64
