# XRES-009: Loaded resources differ from the files

| | |
|---|---|
| Area | XRES |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | a running session (recorded output can be replayed) |
| Fix class | auto (a refresh in the running session) |
| Status | planned |

## What it detects

The resource database of the running session (xrdb on X11, trawld on sway) does not match what the files on disk produce, usually because the user edited a file and did not refresh.

## How it is detected

Compares `xrdb -query` (X11) or `trawldb --query` (sway) with the values the doctor's resolver computes from the same files.

## Fix

Offers `regolith-look refresh`. Auto: the doctor may run it itself because a refresh is harmless.

## Sources

- https://github.com/regolith-linux/regolith-desktop/issues/95
