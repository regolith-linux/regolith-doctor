# INPUT-001: Touchpad settings not applied

| | |
|---|---|
| Area | INPUT |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | report-only |
| Status | planned |

## What it detects

Touchpad settings have no effect: on X11 the Synaptics driver overrides libinput; on sway, user `input` blocks override what regolith-inputd applies.

## How it is detected

The doctor reads the installed packages, the files in `/usr/share/X11/xorg.conf.d` and the `input` blocks in user partials.

## Fix

The doctor explains which setting wins. Report-only: nothing is changed.

## Sources

- https://github.com/regolith-linux/regolith-desktop/issues/1076
- https://github.com/regolith-linux/regolith-desktop/issues/592
- https://github.com/regolith-linux/regolith-desktop/issues/389
- https://github.com/regolith-linux/regolith-desktop/issues/360
