# places-cli
A handy tool for your scripts to get correct path names from known locations or XDG directories.

## Quick usage
```text
Usage: places [OPTIONS] <location> [pathel]...

Arguments:
  <location>

possible values: home, desktop, downloads, templates, documents, pictures,
videos, music, public, config, bin, cache, state, data, autostart, fonts,
menus, backgrounds, icons

  [pathel]...
          Optional list of path elements to append

Options:
  -a, --app <program>
          Optional program to get paths
```
See manpage for more info.

## Examples
- Get a path like: _/home/alice/.config/gizmo/theme/colors.txt_

```shell
places -a gizmo config theme colors.txt \
places config gizmo theme colors.txt
```
- List all config files for the _'gizmo'_ program

```shell
lsd $(places --app=gizmo data) \
places --app=gizmo data | xargs lsd
```

- Copy a config file
```shell
cp gizmo_config.toml $(places -a gizmo config)
```



- Get a folder inside `'Downloads dir'`, even if it is localized.
```shell
places downloads Software
```

This returns something like _/home/alice/Descargas/Software_




