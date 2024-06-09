use clap::{Parser, ValueEnum, ValueHint::CommandName};

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "A handy tool for your scripts to always get correct path names for known locations.\n\
    Sometimes the location is changed by either ENVIRONMENT variables, or by user localization.\n\
    Using this tool you always get the correct path.",
    after_long_help = "Some examples:\n\
        \tGet a path like: /home/alice/.config/gizmo/theme/colors.txt\n\
        \t   $ places -a gizmo config theme colors.txt\n\
        \tor $ places config gizmo theme colors.txt\n\
        \tGet config files:\n\
        \t   $ lsd $(places --app=gizmo data)\n\
        \tor $ places --app=gizmo data | xargs lsd\n\
        \tCopy a config file\n\
        \t   $ cp gizmo_config.toml $(places -a gizmo config)\n\
        \tGet a folder inside Downloads dir, ven if it is Localized\n\
        \t   $ places downloads Software\n\
        \t     /home/alice/Descargas/Software\n\n\
    Report any bugs on github
    "
)]
pub struct Cli {
    #[arg(short, long, value_name = "program", value_hint = CommandName)]
    /// Optional program to get paths
    ///
    /// Example:
    /// '--app lsd config'      ~/.config/lsd
    pub(crate) app: Option<String>,

    #[arg(value_enum, value_name = "location", help = "The (XDG) location.")]
    pub(crate) location: Location,

    /// Optional list of path elements to append
    #[arg(value_name = "pathel")]
    pub(crate) files: Option<Vec<String>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Location {
    #[value(help = "HOME                         /home/alice")]
    Home,
    #[value(help = "XDG_DESKTOP_DIR              /home/alice/Desktop")]
    Desktop,
    #[value(help = "XDG_DOWNLOAD_DIR             /home/alice/Downloads")]
    Downloads,
    #[value(help = "XDG_TEMPLATES_DIR            /home/alice/Templates")]
    Templates,
    #[value(help = "XDG_DOCUMENTS_DIR            /home/alice/Documents")]
    Documents,
    #[value(help = "XDG_PICTURES_DIR             /home/alice/Pictures")]
    Pictures,
    #[value(help = "XDG_VIDEOS_DIR               /home/alice/Videos")]
    Videos,
    #[value(help = "XDG_MUSIC_DIR                /home/alice/Music")]
    Music,
    #[value(help = "XDG_PUBLICSHARE_DIR          /home/alice/Public\n")]
    Public,
    #[value(help = "XDG_CONFIG_HOME               /home/alice/.config")]
    Config,
    #[value(help = "XDG_BIN_HOME                  /home/alice/.local/bin")]
    Bin,
    #[value(help = "XDG_CACHE_HOME                /home/alice/.cache")]
    Cache,
    #[value(help = "XDG_STATE_HOME                /home/alice/.state")]
    State,
    #[value(help = "XDG_DATA_HOME                 /home/alice/.local/share\n\n\
    Note: You always get expanded paths and not paths starting with ~.")]
    Data,
    #[value(help = "XDG_CONFIG_HOME/autostart     ~/.config/autostart")]
    Autostart,
    #[value(help = "XDG_DATA_HOME/fonts           ~/.local/share/fonts")]
    Fonts,
    #[value(help = "XDG_DATA_HOME/applications    ~/.local/share/applications")]
    Menus,
    #[value(help = "XDG_DATA_HOME/backgrounds     ~/.local/share/backgrounds")]
    Backgrounds,
    #[value(help = "XDG_DATA_HOME/icons           ~/.local/share/icons")]
    Icons,
}
