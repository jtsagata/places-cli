[![CI](https://github.com/jtsagata/places-cli/workflows/CI/badge.svg)](https://github.com/jtsagata/places-cli/actions)

# places-cli CLI
A handy tool for your scripts to always get correct path names for known locations.


## What does places-cli?

A handy tool for your scripts to always get correct path names for known locations.
Sometimes the location is changed by either ENVIRONMENT variables, or by user localization.
Using this tool you always get the correct path.


### Motivation
Let's say you want to find the desktop and the download directory.
Without this tool you have to do something like:

```shell
test -f ${XDG_CONFIG_HOME:-~/.config}/user-dirs.dirs && \
     source ${XDG_CONFIG_HOME:-~/.config}/user-dirs.dirs
echo ${XDG_DESKTOP_DIR:-$HOME/Desktop}
echo ${XDG_DOWNLOAD_DIR:-$HOME}
```

with _places_ it becomes:

```shell
places desktop
places downloads
```

Ah, much better

But you can do much more with it

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



Get a folder inside `'Downloads dir'`, even if it is localized.
```shell
places downloads Software
```

This returns something like _/home/alice/Descargas/Software_


### Motivation
Let's say you want to find the desktop and the download directory.
Without this tool you have to do something like:

```shell
test -f ${XDG_CONFIG_HOME:-~/.config}/user-dirs.dirs && \
     source ${XDG_CONFIG_HOME:-~/.config}/user-dirs.dirs
echo ${XDG_DESKTOP_DIR:-$HOME/Desktop}
echo ${XDG_DOWNLOAD_DIR:-$HOME}
```

with _places_ it becomes:

```shell
places desktop
places downloads
```

Ah, much better

But you can do much more with it

- Get a path like: _/home/alice/.config/gizmo/theme/colors.txt_
```shell
$ places -a gizmo config theme colors.txt \
$ places config gizmo theme colors.txt

```
- List all config files for the _'gizmo'_ program
```shell
$ lsd $(places --app=gizmo data) \
$ places --app=gizmo data | xargs lsd
```

- Copy a config file
```shell
$ cp gizmo_config.toml $(places -a gizmo config)
```



Get a folder inside `'Downloads dir'`, even if it is localized.
```shell
$ places downloads Software
```

This returns something like _/home/alice/Descargas/Software_

### Install

To install the places-cli, you just need to run

```bash
cargo install --force places-cli
```

> --`force` just makes it update to the latest `places-cli` if it's already installed \
> The binary is called `places` (without `-cli`)

to verify if the installation was successful, you can run
```shell
which places
```
 that should output similar to

```shell
$HOME/.cargo/bin/places
```

## License

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
