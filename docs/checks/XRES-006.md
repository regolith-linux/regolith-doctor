# XRES-006: Selected look is not installed

| | |
|---|---|
| Area | XRES |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | guided |
| Status | planned |

## What it detects

`regolith.look.path` names a look directory, or a `root` file inside it, that does not exist. The session then falls back to the newest installed look without saying so; older i3xrocks versions crashed with exit code 139.

## How it is detected

Resolves the effective `regolith.look.path` (user files first, then the default-look rule), checks the directory and its `root` file under the root, and maps the directory name to a `regolith-look-<name>` package.

## Fix

Offers `apt install regolith-look-<name>` or `regolith-look set default`. Guided: shown, and run only after confirmation.

## Sources

- https://github.com/regolith-linux/regolith-desktop/issues/1042
- https://github.com/regolith-linux/regolith-desktop/issues/728
- https://github.com/orgs/regolith-linux/discussions/673
