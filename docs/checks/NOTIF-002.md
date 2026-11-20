# NOTIF-002: More than one rofication-daemon

| | |
|---|---|
| Area | NOTIF |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | a running session (recorded output can be replayed) |
| Fix class | auto (live action) |
| Status | planned |

## What it detects

Two rofication daemons run at once. The one that receives notifications is not the one the viewer and the bar ask, so notifications seem to vanish.

## How it is detected

With a running session the doctor checks the process list and the queued owners of the D-Bus name.

## Fix

The doctor offers to restart a single daemon. Auto: the restart is harmless and changes no files.

## Sources

- https://github.com/regolith-linux/regolith-desktop/issues/640
