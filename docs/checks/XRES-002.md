# XRES-002: Renamed or removed resource key

| | |
|---|---|
| Area | XRES |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | auto (one-to-one renames only; reversible) |
| Status | planned |

## What it detects

Resource keys whose name changed between releases, for example `i3-wm.*` to `wm.*`, `gnome.wallpaper` to `regolith.wallpaper.file`, `wm.program.lock` to the session-specific lock keys, and renamed autostart keys. The old names are ignored without any warning.

## How it is detected

Applies a rename table shipped with the doctor (old key, new key, release) to the user resource files and to `~/.Xresources`.

## Fix

Rewrites the key name in place and keeps a backup. Auto: the doctor may do this itself for one-to-one renames because the edit is reversible; other cases are reported.

## Sources

- https://github.com/regolith-linux/website/issues/178
- https://github.com/regolith-linux/regolith-powerd/issues/3
- https://github.com/regolith-linux/regolith-i3-config/issues/8
- https://github.com/regolith-linux/regolith-desktop/issues/597
