# LOOK-001: Theme, icon theme or cursor theme that is not installed

| | |
|---|---|
| Area | LOOK |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | guided |
| Status | planned |

## What it detects

A GTK, icon or cursor theme name, set in GSettings or by the look, that has no matching theme directory. Symptoms are unthemed apps, a slow or hanging ilia, or two different cursors.

## How it is detected

Takes the effective GSettings values for `org.gnome.desktop.interface` `gtk-theme`, `icon-theme` and `cursor-theme` (user dconf value, else Regolith override, else schema default) and the look's `gtk.theme_name` style values, and looks for a theme directory with an `index.theme` under `/usr/share/themes`, `/usr/share/icons` or `~/.local/share`.

## Fix

Offers the look's value, or names the package that ships the theme. Guided: applied only after confirmation.

## Sources

- https://github.com/regolith-linux/regolith-desktop/issues/875
- https://github.com/regolith-linux/regolith-i3xrocks-config/issues/147
- https://github.com/orgs/regolith-linux/discussions/684
- https://github.com/orgs/regolith-linux/discussions/1163
