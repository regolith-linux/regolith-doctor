# COMP-002: A second compositor starts from the distribution's autostart

| | |
|---|---|
| Area | COMP |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | auto |
| Status | planned |

## What it detects

`/etc/xdg/autostart/picom.desktop` starts the stock picom next to Regolith's own compositor on X11, and it brings down the sway session.

## How it is detected

The doctor looks for compositor entries in `/etc/xdg/autostart` and `~/.config/autostart` that are not hidden (`Hidden=true`) and not excluded from Regolith with `NotShowIn`.

## Fix

The doctor writes `~/.config/autostart/picom.desktop` with `Hidden=true`. Auto: the change stays inside HOME, is backed up and can be undone.

## Sources

- https://github.com/regolith-linux/regolith-session/issues/39
- https://github.com/regolith-linux/regolith-desktop/issues/1024
- https://github.com/orgs/regolith-linux/discussions/877
