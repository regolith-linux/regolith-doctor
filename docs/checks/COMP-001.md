# COMP-001: GLX compositor on a machine where it misbehaves

| | |
|---|---|
| Area | COMP |
| Sessions | i3 (X11) |
| Releases | Regolith 3.4, 3.5 |
| Needs | files only |
| Fix class | guided |
| Status | planned |

## What it detects

The GLX picom compositor runs in a virtual machine (VMware, VirtualBox, QEMU/SPICE, ESXi), on software rendering, or on NVIDIA without the sync options. Users see flicker, black or white windows, or input lag that feels like dropped keys.

## How it is detected

The doctor reads which `regolith-compositor-*` package is installed, the DMI vendor and product (`/sys/class/dmi/id/sys_vendor`, `/sys/class/dmi/id/product_name`) and the loaded kernel modules in `/proc/modules` (nvidia, vmwgfx, virtio_gpu). When available it also uses the renderer from `glxinfo -B`.

## Fix

The doctor suggests `regolith-compositor-none` or xrender settings. Guided: the change is shown and applied only after you confirm.

## Sources

- https://github.com/regolith-linux/regolith-desktop/issues/1072
- https://github.com/regolith-linux/regolith-desktop/issues/559
- https://github.com/regolith-linux/regolith-desktop/issues/510
- https://github.com/regolith-linux/regolith-desktop/issues/506
