# XRES-003: Resource file with the wrong name or in the wrong place

| | |
|---|---|
| Area | XRES |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | guided |
| Status | planned |

## What it detects

Resource files the session never loads because of their name or location: `~/.config/regolith3/.Xresources` with a leading dot, a misspelled `Xresources.d`, files left under `regolith2`.

## How it is detected

Compares the files found in HOME with the ones regolith-session loads: `~/.Xresources`, `~/.config/regolith3/Xresources` and the includes from `~/.config/regolith3/Xresources.d`.

## Fix

Offers to rename the file or merge it into the right one. Guided: applied only after confirmation.

## Sources

- https://github.com/orgs/regolith-linux/discussions/889
