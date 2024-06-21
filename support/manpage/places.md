---
title: places
section: 1
header: User Manual
footer: places <version>
date: <date>
---

# NAME

places - Get paths for common places

# SYNOPSIS

**places** \[**OPTIONS**] <**location**> \[**pathel**]...

# DESCRIPTION
A handy tool for your scripts to get correct path names from known locations or XDG directories.


# ARGUMENTS
 `<location>`
: The  location. See table bellow for a list of known locations.


`[pathel]...`
: Optional list of path elements to append

# OPTIONS

`-a`, `--app`
: Optional program to get paths

`-h`, `--help`
: Print help (see a summary with `-h`)

`-V`, `-version`
: Print version

# LOCATION TABLE
The following is the list of known locations

### **XDG** locations

| name          | environment var      | example                 |
|---------------|----------------------|-------------------------|
| **home**      | $HOME                | _/home/alice_           |
| **desktop**   | $XDG_DESKTOP_DIR     | _/home/alice/Desktop_   |
| **downloads** | $XDG_DOWNLOAD_DIR    | _/home/alice/Downloads_ |
| **templates** | $XDG_TEMPLATES_DIR   | _/home/alice/Templates_ |
| **documents** | $XDG_DOCUMENTS_DIR   | _/home/alice/Documents_ |
| **pictures**  | $XDG_PICTURES_DIR    | _/home/alice/Pictures_  |
| **videos**    | $XDG_VIDEOS_DIR      | _/home/alice/Videos_    |
| **music**     | $XDG_MUSIC_DIR       | _/home/alice/Music_     |
| **public**    | $XDG_PUBLICSHARE_DIR | _/home/alice/Public_    |

### Common locations


| name       | environment var  | example                    |
|------------|------------------|----------------------------|
| **config** | $XDG_CONFIG_HOME | _/home/alice/.config_      |
| **bin**    | $XDG_BIN_HOME    | _/home/alice/.local/bin_   |
| **cache**  | $XDG_CACHE_HOME  | _/home/alice/.cache_       |
| **state**  | $XDG_STATE_HOME  | _/home/alice/.state_       |
| **data**   | $XDG_DATA_HOME   | _/home/alice/.local/share_ |

### Other locations
Note the tool always return a full path, not paths starting with _'~/'_.

| name            | example                       |
|-----------------|-------------------------------|
| **autostart**   | _~/.config/autostart_         |
| **fonts**       | _~/.local/share/fonts_        |
| **menus**       | _~/.local/share/applications_ |
| **backgrounds** | _~/.local/share/backgrounds_  |


# EXAMPLES

But you can do much more with it

### * Get a path like: _/home/alice/.config/gizmo/theme/colors.txt_
   **\$** places -a gizmo config theme colors.txt \
   **\$** places config gizmo theme colors.txt

### * List all config files for the _'gizmo'_ program

   **\$** lsd $(places --app=gizmo data) \
   **\$** places --app=gizmo data | xargs lsd

### * Copy a config file
   **\$** cp gizmo_config.toml $(places -a gizmo config)


### * Get a folder inside `'Downloads dir'`, even if it is localized.
   **\$** places downloads Software

   This returns something like _/home/alice/Descargas/Software_


# ENVIRONMENT
The list of environment variables that affect the operation is listed on the tables above.

The **XDG** localized names is defined in the file _/etc/xdg/user-dirs.defaults_ and for each user
in the file _$(XDG_CONFIG_HOME)/user-dirs.dirs_.


# BUGS

See GitHub Issues: _https://github.com/jtsagata/places-cli/issues_

# SEE ALSO
**xdg-user-dir(1)**, **user-dirs.dirs(5)**
