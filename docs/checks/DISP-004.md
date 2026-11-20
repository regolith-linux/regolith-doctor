# DISP-004: Phantom outputs

| | |
|---|---|
| Area | DISP |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | a running session (recorded output can be replayed) |
| Fix class | report-only |
| Status | planned |

## What it detects

The window manager still reports outputs that are disconnected or unknown (seen with NVIDIA and headless setups), and those outputs take workspaces that then cannot be reached.

## How it is detected

The doctor looks in the live `get_outputs` reply for outputs with zero size, an `<Unknown>` name, or inactive outputs that still hold workspaces.

## Fix

The doctor lists the phantom outputs and the workspaces on them. Report-only: nothing is changed.

## Sources

- https://github.com/regolith-linux/regolith-control-center/issues/10
- https://github.com/regolith-linux/regolith-desktop/issues/636
- https://github.com/regolith-linux/regolith-desktop/issues/609
- https://github.com/regolith-linux/regolith-desktop/issues/136
