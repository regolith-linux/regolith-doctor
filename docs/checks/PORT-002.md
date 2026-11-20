# PORT-002: Portal does not answer FileChooser or Settings requests

| | |
|---|---|
| Area | PORT |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | a running session (recorded output can be replayed) |
| Fix class | report-only |
| Status | planned |

## What it detects

`org.freedesktop.portal.Desktop` cannot be activated, or reading the colour scheme through the Settings interface fails, so dialogs and dark mode break in sandboxed and GTK4 applications.

## How it is detected

With a running session the doctor introspects the portal on D-Bus and calls it, with timeouts.

## Fix

The doctor reports which call failed. Report-only: nothing is changed.

## Sources

- https://github.com/regolith-linux/regolith-desktop/issues/1114
- https://github.com/regolith-linux/xdg-desktop-portal-regolith/issues/17
