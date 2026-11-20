# NOTIF-001: Another notification daemon takes the notifications

| | |
|---|---|
| Area | NOTIF |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | guided |
| Status | planned |

## What it detects

dunst, mako, xfce4-notifyd or notify-osd owns `org.freedesktop.Notifications`, so rofication never sees a notification; or no notification daemon exists at all.

## How it is detected

The doctor reads D-Bus service files that declare `Name=org.freedesktop.Notifications` in `/usr/share/dbus-1/services` and the autostart entries. With a running session it asks D-Bus for the current owner (`GetNameOwner`).

## Fix

The doctor explains which daemon wins and offers to hide its autostart entry. Guided: the change is shown and applied only after you confirm.

## Sources

- https://github.com/regolith-linux/regolith-desktop/issues/1068
- https://github.com/regolith-linux/regolith-desktop/issues/1025
- https://github.com/regolith-linux/regolith-desktop/issues/723
- https://github.com/regolith-linux/regolith-desktop/issues/621
