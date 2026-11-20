# XRES-001: Unknown or misspelled resource key

| | |
|---|---|
| Area | XRES |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | guided |
| Status | planned |

## What it detects

Keys in `~/.config/regolith3/Xresources` (and the files it includes from `Xresources.d`) that nothing reads: typos such as `i3-wm-mod`, keys copied from the docs of another release, bar font keys that do not exist. The setting is silently ignored.

## How it is detected

Builds a catalogue of known keys from every `set_from_resource` under `/usr/share/regolith` and `/etc/regolith`, the look files, and a table shipped with the doctor of keys read by programs (regolith-session, powerd, inputd, displayd, i3xrocks, ilia). Each user key not in the catalogue is reported with the closest known key (edit distance, same prefix).

## Fix

Reports the key with a suggested replacement and renames it on request. Guided: renamed only after confirmation.

## Sources

- https://github.com/regolith-linux/regolith-desktop/issues/469
- https://github.com/regolith-linux/regolith-desktop/issues/408
- https://github.com/regolith-linux/regolith-desktop/issues/295
- https://github.com/regolith-linux/regolith-desktop/issues/72
