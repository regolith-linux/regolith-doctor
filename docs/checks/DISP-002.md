# DISP-002: DPI and scaling settings disagree

| | |
|---|---|
| Area | DISP |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | report-only |
| Status | planned |

## What it detects

The DPI in resources (`Xft.dpi`) does not match the GNOME text scaling or scaling factor, the X11 bar renders at a different scale than applications, or the sway output scale does not fit the cursor size. Text and widgets end up with mixed sizes.

## How it is detected

The doctor compares the effective resources with the GSettings scaling values and, when a session is running, with the live output scales.

## Fix

The doctor reports a consistent set of values to use. Report-only: nothing is changed.

## Sources

- https://github.com/orgs/regolith-linux/discussions/722
- https://github.com/regolith-linux/regolith-desktop/issues/534
- https://github.com/regolith-linux/regolith-desktop/issues/491
- https://github.com/regolith-linux/regolith-desktop/issues/417
