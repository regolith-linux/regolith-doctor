# LOOK-005: Wallpaper set in GNOME will be replaced at next login

| | |
|---|---|
| Area | LOOK |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | guided |
| Status | planned |

## What it detects

The user set `org.gnome.desktop.background picture-uri`, but it differs from `regolith.wallpaper.file` or the look's wallpaper, so the next login or `regolith-look refresh` resets it. Also custom images saved into package-owned paths, which an upgrade puts back.

## How it is detected

Compares the effective GSettings value with the resolved wallpaper resources, and checks whether a wallpaper path under `/usr/share/backgrounds/regolith*` is owned by a package (dpkg file lists).

## Fix

Offers the `regolith.wallpaper.file` line for the user's resource file. Guided: written only after confirmation.

## Sources

- https://github.com/regolith-linux/regolith-desktop/issues/630
- https://github.com/regolith-linux/regolith-desktop/issues/623
- https://github.com/regolith-linux/regolith-desktop/issues/421
- https://github.com/regolith-linux/regolith-desktop/issues/344
