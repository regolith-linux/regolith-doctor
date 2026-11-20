# LOOK-002: Look not applied to GTK settings

| | |
|---|---|
| Area | LOOK |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | guided (could be auto on request) |
| Status | planned |

## What it detects

GSettings theme, icon, font or colour-scheme values that differ from the active look. Typical causes: `~/.config/regolith3/flags` exists (for example restored from a backup), so the loader skipped its first-run setup; a start-order bug in 3.4; or a look change without a refresh.

## How it is detected

Compares the effective GSettings with the look's `gtk.*` and `gnome.*` resources, and notes whether the `flags` directory exists.

## Fix

Offers to run `regolith-look refresh`. Guided: run only after confirmation.

## Sources

- https://github.com/orgs/regolith-linux/discussions/1158
- https://github.com/regolith-linux/regolith-desktop/issues/1119
- https://github.com/regolith-linux/regolith-look-extra/issues/19
- https://github.com/regolith-linux/regolith-desktop/issues/649
