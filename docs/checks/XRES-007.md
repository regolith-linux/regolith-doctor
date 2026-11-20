# XRES-007: Workspace names that break workspace keys

| | |
|---|---|
| Area | XRES |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | guided |
| Status | planned |

## What it detects

`wm.workspace.NN.name` values without the `N:` prefix, so sway cannot switch to the workspace by number, and names containing characters that need quoting.

## How it is detected

Parses the effective resources and checks every workspace name for the number prefix and the quoting rules.

## Fix

Shows the corrected value and writes it on request. Guided: applied only after confirmation.

## Sources

- https://github.com/orgs/regolith-linux/discussions/1053
- https://github.com/regolith-linux/regolith-desktop/issues/909
- https://github.com/orgs/regolith-linux/discussions/691
- https://github.com/regolith-linux/regolith-desktop/issues/614
