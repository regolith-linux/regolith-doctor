# COMP-003: User compositor config is stale or in a place that is not read

| | |
|---|---|
| Area | COMP |
| Sessions | i3 (X11) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | report-only |
| Status | planned |

## What it detects

The user keeps a full copy of an old picom config that misses later fixes, keeps it under `regolith2`, or expects `~/.config/picom.conf` to apply. The compositor ignores the change or keeps old bugs.

## How it is detected

The doctor works out which file the Regolith start script reads (`~/.config/regolith3/picom/config`, otherwise `/etc/regolith/picom/config`), compares a user copy with the shipped config, and flags deprecated options.

## Fix

The doctor shows the difference and where the file should live. Report-only: nothing is changed.

## Sources

- https://github.com/regolith-linux/regolith-look-extra/issues/22
- https://github.com/regolith-linux/regolith-desktop/issues/665
- https://github.com/regolith-linux/regolith-desktop/issues/654
- https://github.com/regolith-linux/regolith-compositor-picom-glx/issues/2
