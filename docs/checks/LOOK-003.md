# LOOK-003: Dark mode does not reach GTK4 and libadwaita apps

| | |
|---|---|
| Area | LOOK |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | guided |
| Status | planned |

## What it detects

The look is dark, but `color-scheme` is not `prefer-dark`, or no portal answers the `org.freedesktop.appearance` `color-scheme` setting, so GTK4 and libadwaita apps stay light.

## How it is detected

From files: compares how dark the look is (a look key, or the background colour's luminance) with the `color-scheme` value. In a running session, also reads the setting through the portal (see PORT-002).

## Fix

Sets `color-scheme` and keeps the old value for undo. Guided: applied only after confirmation.

## Sources

- https://github.com/regolith-linux/xdg-desktop-portal-regolith/issues/21
- https://github.com/regolith-linux/xdg-desktop-portal-regolith/issues/17
- https://github.com/regolith-linux/regolith-desktop/issues/1114
- https://github.com/orgs/regolith-linux/discussions/1044
