# XRES-004: Resource file the preprocessor or parser mangles

| | |
|---|---|
| Area | XRES |
| Sessions | i3 (X11) and sway (Wayland) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | auto for comment markers and `~` in includes; report-only otherwise |
| Status | planned |

## What it detects

Lines that the C preprocessor or the resource parser read differently from what the user meant: `#` comments that look like directives (resource comments start with `!`), `~` or `$HOME` in an `#include` (not expanded), include targets that do not exist, lines without a colon, `bindsym` lines pasted into Xresources, values with unbalanced quotes.

## How it is detected

A line-by-line lint that follows the rules of `regolith-xres-cpp` and xrdb: directive recognition, include resolution with the `-I` search path, the `name: value` grammar and continuation lines.

## Fix

Changes `#` comments to `!` and expands `~` in include paths; everything else is reported. Auto for those two mechanical edits, which are reversible; report-only for the rest.

## Sources

- https://github.com/regolith-linux/regolith-desktop/issues/819
- https://github.com/regolith-linux/website/issues/155
- https://github.com/regolith-linux/regolith-desktop/issues/99
- https://github.com/regolith-linux/regolith-desktop/issues/101
