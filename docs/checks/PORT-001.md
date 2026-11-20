# PORT-001: Portal configuration picks a backend that is not installed

| | |
|---|---|
| Area | PORT |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | guided |
| Status | planned |

## What it detects

The file chooser or screen sharing does not work, or applications take 25-50 seconds to start, because a `*-portals.conf` file names a backend with no `.portal` file, xdg-desktop-portal-gnome times out, or no wlr backend exists for screen sharing on sway.

## How it is detected

The doctor reads `/usr/share/xdg-desktop-portal/*-portals.conf`, `~/.config/xdg-desktop-portal/` and the backend files `/usr/share/xdg-desktop-portal/portals/*.portal` (`UseIn`, `Interfaces`), and matches them against the `XDG_CURRENT_DESKTOP` value of each Regolith session.

## Fix

The doctor names the backend package to install or the config line to change. Guided: the change is shown and applied only after you confirm.

## Sources

- https://github.com/regolith-linux/regolith-desktop/issues/1066
- https://github.com/orgs/regolith-linux/discussions/1009
- https://github.com/orgs/regolith-linux/discussions/984
- https://github.com/regolith-linux/regolith-desktop/issues/920
