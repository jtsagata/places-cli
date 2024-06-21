[![Crates.io](https://img.shields.io/crates/v/places-cli)](https://crates.io/crates/places-cli)
[![GitHub Release](https://img.shields.io/github/v/release/jtsagata/places-cli)](https://github.com/jtsagata/places-cli/releases)
[![CI](https://github.com/jtsagata/places-cli/workflows/CI/badge.svg)](https://github.com/jtsagata/places-cli/actions)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

# places-cli
A handy tool for your scripts to get correct path names from known locations or XDG directories.

**NOTE**: The package name is `places-cli` and the binary name is just `places`.

### Motivation
Let's say you want to find the desktop and the download directory.
Without this tool you have to do something like:

```shell
test -f ${XDG_CONFIG_HOME:-~/.config}/user-dirs.dirs && \
     source ${XDG_CONFIG_HOME:-~/.config}/user-dirs.dirs
echo ${XDG_DESKTOP_DIR:-$HOME/Desktop}
echo ${XDG_DOWNLOAD_DIR:-$HOME}
```

Or use something `xdg-user-dir DOWNLOAD`, but this is prone to errors.  With _places_ it becomes:

```shell
places desktop
places downloads
```

Ah, much better, no UPPERCASE and will return an error if you make some typo.
But wait, you can do much more with it

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


### Installation

To install the places-cli, you just need to run

```bash
cargo install --force places-cli
```

> --`force` just makes it update to the latest `places-cli` if it's already installed

Download the man page and the completions for github repository and put them in the right place.

Next version will have an installation script and hopefully some deb packages and rpms (help wanted with github actions).


#### Verifiy the installation
To verify if the installation was successful, you can run
```shell
which places
```
that should output similar to

```shell
$HOME/.cargo/bin/places
```

