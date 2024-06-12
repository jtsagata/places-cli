# places-cli CLI
A handy tool for your scripts to always get correct path names for known locations.


## What does places-cli?

A handy tool for your scripts to always get correct path names for known locations.
Sometimes the location is changed by either ENVIRONMENT variables, or by user localization.
Using this tool you always get the correct path.

### Install

To install the places-cli, you just need to run

```bash
cargo install --force places-cli
```

(--force just makes it update to the latest `places-cli` if it's already installed)

**Note** the binary is called `places` (without `-cli`)

to verify if the installation was successful, you can run `which places` that should output similar to

```sh
$HOME/.cargo/bin/places
```

### Usage

# Quick Intro

```
A handy tool for your scripts to always get correct path names for known locations.
Sometimes the location is changed by either ENVIRONMENT variables, or by user localization.
Using this tool you always get the correct path.

Usage: places [OPTIONS] <location> [pathel]...

Arguments:
  <location>
          The (XDG) location.

          Possible values:
          - home:        HOME                         /home/alice
          - desktop:     XDG_DESKTOP_DIR              /home/alice/Desktop
          - downloads:   XDG_DOWNLOAD_DIR             /home/alice/Downloads
          - templates:   XDG_TEMPLATES_DIR            /home/alice/Templates
          - documents:   XDG_DOCUMENTS_DIR            /home/alice/Documents
          - pictures:    XDG_PICTURES_DIR             /home/alice/Pictures
          - videos:      XDG_VIDEOS_DIR               /home/alice/Videos
          - music:       XDG_MUSIC_DIR                /home/alice/Music
          - public:      XDG_PUBLICSHARE_DIR          /home/alice/Public
            
          - config:      XDG_CONFIG_HOME               /home/alice/.config
          - bin:         XDG_BIN_HOME                  /home/alice/.local/bin
          - cache:       XDG_CACHE_HOME                /home/alice/.cache
          - state:       XDG_STATE_HOME                /home/alice/.state
          - data:        XDG_DATA_HOME                 /home/alice/.local/share
            
            Note: You always get expanded paths and not paths starting with ~.
          - autostart:   XDG_CONFIG_HOME/autostart     ~/.config/autostart
          - fonts:       XDG_DATA_HOME/fonts           ~/.local/share/fonts
          - menus:       XDG_DATA_HOME/applications    ~/.local/share/applications
          - backgrounds: XDG_DATA_HOME/backgrounds     ~/.local/share/backgrounds
          - icons:       XDG_DATA_HOME/icons           ~/.local/share/icons

  [pathel]...
          Optional list of path elements to append

Options:
  -a, --app <program>
          Optional program to get paths
          
          Example: '--app lsd config'      ~/.config/lsd

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

Some examples:
	Get a path like: /home/alice/.config/gizmo/theme/colors.txt
	   $ places -a gizmo config theme colors.txt
	or $ places config gizmo theme colors.txt
	Get config files:
	   $ lsd $(places --app=gizmo data)
	or $ places --app=gizmo data | xargs lsd
	Copy a config file
	   $ cp gizmo_config.toml $(places -a gizmo config)
	Get a folder inside Downloads dir, ven if it is Localized
	   $ places downloads Software
	     /home/alice/Descargas/Software

```

### TODO
- [x] Basic functionality
- [ ] Test in production
- [ ] Add license and docs
- [ ] Man page, completion
- [ ] Debian packaging
- [ ] Release and install script

## License

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

- **[GNU GPL v3 license](https://www.gnu.org/licenses/gpl-3.0)**
- Copyright 2019 © ["Tsagatakis Yianis][me].

[me]: https://linux-user.gr
