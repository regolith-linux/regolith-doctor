# COMP-004: Compositor not running or restarting in a loop

| | |
|---|---|
| Area | COMP |
| Sessions | i3 (X11) |
| Releases | Regolith 3.4, 3.5 |
| Needs | a running session (recorded output can be replayed) |
| Fix class | auto (live action) |
| Status | planned |

## What it detects

picom has exited and was not started again, or it keeps crashing and restarting.

## How it is detected

The doctor checks the state of the compositor's user unit, the process list and the user journal of the running session.

## Fix

The doctor offers to restart the compositor. Auto: restarting is harmless and changes no files.

## Sources

- https://github.com/regolith-linux/regolith-compositor-picom-glx/issues/4
