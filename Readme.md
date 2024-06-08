# Base Dirs

| Function    | notes| Example|
|-------------|-----------------|--------|
| home_dir ()      | $HOME            | /home/alice/ |
| * cache_dir()      | $XDG_CACHE_HOME  | /home/alice/.cache |
| * config_dir()     | $XDG_CONFIG_HOME | /home/alice/.config |
| * data_dir()       | $XDG_DATA_HOME   | /home/alice/.local/share |
| * preference_dir() | $XDG_CONFIG_HOME | /home/alice/.config |
| ? executable_dir | $XDG_BIN_HOME    | /home/alice/.local/bin |

1. The state directory contains data that should be retained between sessions (unlike the runtime directory), but may not be important/portable enough to be synchronized across machines (unlike the config/preferences/data directories).
2. It is safe to assume that constructor works
3. cat ~/.config/user-dirs.dirs 

places xdg [directory] 
places base [directory] 
places <project name> [directory] [<filename>] 

places [--program <name>] [audio,home,bin,config] <filename>

1. Leave the making links can be done in bash  [--make]|[--link <file>] 
2. Move xdg to separate executable

