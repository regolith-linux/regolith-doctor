# LNCH-002: Desktop entries the launcher cannot see

| | |
|---|---|
| Area | LNCH |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | guided |
| Status | planned |

## What it detects

Applications are missing from the launcher because `XDG_DATA_DIRS` lacks the flatpak, nix or home-manager export folders, because the entry is a symlink, or because `NoDisplay` or `OnlyShowIn` hides it.

## How it is detected

The doctor reads the environment files and the data folders under the root, and parses the desktop entries in them.

## Fix

The doctor offers the `environment.d` line that adds the missing folder. Guided: the change is shown and applied only after you confirm.

## Sources

- https://github.com/orgs/regolith-linux/discussions/1078
- https://github.com/regolith-linux/ilia/issues/35
- https://github.com/orgs/regolith-linux/discussions/1124
- https://github.com/orgs/regolith-linux/discussions/696
