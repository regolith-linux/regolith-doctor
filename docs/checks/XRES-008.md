# XRES-008: `~/.Xresources` overrides Regolith settings the user forgot

| | |
|---|---|
| Area | XRES |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | guided |
| Status | planned |

## What it detects

Regolith keys (`wm.*`, `regolith.*`, `i3xrocks.*`) in `~/.Xresources`, which is loaded first and is easy to forget, and empty `wm.binding.*` values that leave a key unbound.

## How it is detected

Parses `~/.Xresources` and looks for Regolith key prefixes and empty values.

## Fix

Offers to move those lines into `~/.config/regolith3/Xresources`. Guided: moved only after confirmation.

## Sources

- https://github.com/regolith-linux/regolith-desktop/issues/997
