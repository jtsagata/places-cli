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

TODO:Some examples
```

### TODO
- [x] Basic functionality
- [ ] Test in production
- [ ] Add license and docs
- [ ] Man page, completion
- [ ] Debian packaging
- [ ] Release and install script