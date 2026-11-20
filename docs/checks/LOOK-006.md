# LOOK-006: Plain GNOME session keeps Regolith's look settings

| | |
|---|---|
| Area | LOOK |
| Sessions | GNOME session (after using Regolith) |
| Releases | Regolith 3.4, 3.5 (see https://github.com/regolith-linux/regolith-session/pull/111) |
| Needs | files only |
| Fix class | guided |
| Status | planned |

## What it detects

After using Regolith, the plain GNOME session shows Regolith's theme, fonts and wallpaper, because the look wrote them into the user's GSettings.

## How it is detected

Compares the user's dconf values with the look's values, and checks whether a GNOME session is installed as well.

## Fix

Offers a "restore GNOME defaults" action that resets only the keys Regolith set, with a backup. Guided: applied only after confirmation.

## Sources

- https://github.com/regolith-linux/regolith-desktop/issues/474
- https://github.com/orgs/regolith-linux/discussions/1067
