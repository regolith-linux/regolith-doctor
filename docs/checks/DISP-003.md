# DISP-003: Workspace-to-output rules name outputs that do not exist

| | |
|---|---|
| Area | DISP |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | a running session (recorded output can be replayed) |
| Fix class | report-only |
| Status | planned |

## What it detects

`wm.workspace.NN.output` resources or `workspace N output X` lines name connectors that are not present, so workspaces pile up on one screen or get duplicated.

## How it is detected

The doctor compares the rules with the outputs the running window manager reports (`get_outputs`).

## Fix

The doctor lists the rules and the outputs that actually exist. Report-only: nothing is changed.

## Sources

- https://github.com/regolith-linux/regolith-desktop/issues/605
