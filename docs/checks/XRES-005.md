# XRES-005: Resource value has the wrong type

| | |
|---|---|
| Area | XRES |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | report-only |
| Status | planned |

## What it detects

Values of the wrong kind: `yes` where `true` is expected, colours that are not `#rrggbb`, numbers with units, font lists the window manager rejects with "Invalid font specification".

## How it is detected

Infers the type of each catalogue key (boolean, integer, colour, font, command) from its shipped default and checks the user's value against it.

## Fix

Reports the value and the expected form. Report-only: the doctor changes nothing.

## Sources

- https://github.com/regolith-linux/regolith-desktop/issues/283
