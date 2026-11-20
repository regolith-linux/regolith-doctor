# LNCH-001: Programs on the user's PATH are not on the session's PATH

| | |
|---|---|
| Area | LNCH |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | guided |
| Status | planned |

## What it detects

PATH is extended only in `~/.profile` or `~/.bashrc`, or `/etc/environment` resets it, so directories such as `~/.local/bin`, `/snap/bin` or cargo and go bin folders are missing from the graphical session. ilia hides desktop entries whose program it cannot find, and bindings fail.

## How it is detected

The doctor compares PATH additions in shell startup files with `~/.config/environment.d/*.conf`, and lists desktop entries whose `TryExec` or `Exec` only resolves through the shell PATH. With a running session it reads `systemctl --user show-environment`.

## Fix

The doctor offers an `environment.d` snippet that adds the missing directories. Guided: the change is shown and applied only after you confirm.

## Sources

- https://github.com/regolith-linux/ilia/issues/95
- https://github.com/regolith-linux/regolith-desktop/issues/958
- https://github.com/regolith-linux/ilia/issues/41
- https://github.com/regolith-linux/ilia/issues/30
