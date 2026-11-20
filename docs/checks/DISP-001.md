# DISP-001: Stale or broken kanshi profiles

| | |
|---|---|
| Area | DISP |
| Sessions | sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | guided |
| Status | planned |

## What it detects

kanshi profiles name outputs that are never connected, use `<Unknown>` descriptors, or include files outside HOME or from another user. Nothing gets arranged when a dock is plugged in or removed.

## How it is detected

The doctor parses the kanshi config and profiles at the `kanshi.path` resource (default `~/.config/regolith3/kanshi`) and checks descriptors and include paths. With a running session it also compares the profiles with `swaymsg -t get_outputs` (make, model, serial) and looks for 'no profile matched' in the `regolith-init-kanshi` user journal.

## Fix

The doctor offers to move stale profiles into `profiles-on-hold`, the folder regolith-displayd also uses. Guided: the change is shown and applied only after you confirm.

## Sources

- https://github.com/orgs/regolith-linux/discussions/1144
- https://github.com/orgs/regolith-linux/discussions/1120
- https://github.com/regolith-linux/regolith-desktop/issues/52
