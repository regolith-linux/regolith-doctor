# LOOK-004: Font named in the bar, look or Xresources is not installed

| | |
|---|---|
| Area | LOOK |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | programs installed on the system, no running session |
| Fix class | report-only (guided install) |
| Status | planned |

## What it detects

`wm.bar.font`, a `bar { font ... }` line or `gtk.font_name` names a font family that fontconfig does not have. Icons show as boxes and text falls back to DejaVu.

## How it is detected

Collects the family names from those places, lists the installed families (`fc-list : family`, or a scan of the font directories) and runs `fc-match` on each family to see whether it falls back to another font.

## Fix

Names the package that provides the font, such as `fonts-font-awesome` or a Nerd Font. Report-only; the install is offered as a guided step.

## Sources

- https://github.com/regolith-linux/regolith-look-extra/issues/10
- https://github.com/regolith-linux/remontoire/issues/16
- https://github.com/regolith-linux/regolith-st/issues/7
