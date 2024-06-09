use clap::ValueHint::CommandName;
use clap::{Parser, ValueEnum};
use directories::{BaseDirs, ProjectDirs, UserDirs};
use std::path::{Path, PathBuf};

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
struct Cli {
    #[arg(short, long, value_name = "program", value_hint = CommandName)]
    /// Optional program to get paths
    ///
    /// Example:
    /// '--app lsd config'      ~/.config/lsd
    app: Option<String>,

    #[arg(value_enum, value_name = "location", help = "The (XDG) location.")]
    location: Location,

    /// Optional list of path elements to append
    #[arg(value_name = "pathel")]
    files: Option<Vec<String>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Location {
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

fn main() {
    let cli = Cli::parse();
    let mut base_dir = PathBuf::from(match cli.app {
        None => get_home_dir(&cli.location),
        Some(app) => get_app_dir(&app, &cli.location),
    });

    if let Some(paths) = cli.files {
        for p in paths {
            base_dir.push(p)
        }
    }

    println!("{}", base_dir.display());
}

fn get_app_dir(app: &String, location: &Location) -> String {
    let user_dirs = UserDirs::new().unwrap();
    let base_dirs = BaseDirs::new().unwrap();
    let app_dirs = ProjectDirs::from("", "", app).unwrap();

    match location {
        // It is expected to allways get something, so just called unwraped until something bad reported
        Location::Home => as_string(user_dirs.home_dir()),
        Location::Desktop => as_string(&user_dirs.desktop_dir().unwrap().join(&app)),
        Location::Downloads => as_string(&user_dirs.download_dir().unwrap().join(&app)),
        Location::Templates => as_string(&user_dirs.template_dir().unwrap().join(&app)),
        Location::Documents => as_string(&user_dirs.document_dir().unwrap().join(&app)),
        Location::Pictures => as_string(&user_dirs.picture_dir().unwrap().join(&app)),
        Location::Videos => as_string(&user_dirs.video_dir().unwrap().join(&app)),
        Location::Music => as_string(&user_dirs.audio_dir().unwrap().join(&app)),
        Location::Public => as_string(&user_dirs.public_dir().unwrap().join(&app)),
        Location::Config => as_string(app_dirs.config_dir()),
        Location::Bin => as_string(&base_dirs.executable_dir().unwrap().join(&app)),
        Location::Cache => as_string(app_dirs.cache_dir()),
        Location::Data => as_string(app_dirs.data_dir()),
        Location::State => as_string(&base_dirs.state_dir().unwrap().join(&app)),
        // Fall backs for orthogonality, It is like just ignoring the --app flag
        Location::Autostart => as_string(base_dirs.config_dir().join("autostart").as_path()),
        Location::Fonts => as_string(base_dirs.data_dir().join("fonts").as_path()),
        Location::Menus => as_string(base_dirs.data_dir().join("applications").as_path()),
        Location::Backgrounds => as_string(base_dirs.data_dir().join("backgrounds").as_path()),
        Location::Icons => as_string(base_dirs.data_dir().join("icons").as_path()),
    }
}

fn get_home_dir(location: &Location) -> String {
    let user_dirs = UserDirs::new().unwrap();
    let base_dirs = BaseDirs::new().unwrap();

    match location {
        // It is expected to allways get something, so just called unwraped until something bad reported
        Location::Home => as_string(user_dirs.home_dir()),
        Location::Desktop => as_string(user_dirs.desktop_dir().unwrap()),
        Location::Downloads => as_string(user_dirs.download_dir().unwrap()),
        Location::Templates => as_string(user_dirs.template_dir().unwrap()),
        Location::Documents => as_string(user_dirs.document_dir().unwrap()),
        Location::Pictures => as_string(user_dirs.picture_dir().unwrap()),
        Location::Videos => as_string(user_dirs.video_dir().unwrap()),
        Location::Music => as_string(user_dirs.audio_dir().unwrap()),
        Location::Public => as_string(user_dirs.public_dir().unwrap()),
        Location::Config => as_string(base_dirs.config_dir()),
        Location::Bin => as_string(base_dirs.executable_dir().unwrap()),
        Location::Cache => as_string(base_dirs.cache_dir()),
        Location::State => as_string(base_dirs.state_dir().unwrap()),
        Location::Data => as_string(base_dirs.data_dir()),
        Location::Autostart => as_string(base_dirs.config_dir().join("autostart").as_path()),
        Location::Fonts => as_string(base_dirs.data_dir().join("fonts").as_path()),
        Location::Menus => as_string(base_dirs.data_dir().join("applications").as_path()),
        Location::Backgrounds => as_string(base_dirs.data_dir().join("backgrounds").as_path()),
        Location::Icons => as_string(base_dirs.data_dir().join("icons").as_path()),
    }
}

fn as_string(p: &Path) -> String {
    p.display().to_string()
}
