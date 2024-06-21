use clap::{Parser, ValueEnum, ValueHint::CommandName};

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "A handy tool for your scripts to always get correct path names for known locations.\n\
    Sometimes the location is changed by either ENVIRONMENT variables, or by user localization.\n\
    Using this tool you always get the correct path.",
    after_long_help = "See manpage for more info and examples"
)]
pub struct Cli {
    #[arg(short, long, value_name = "program", value_hint = CommandName)]
    /// Optional program to get paths
    pub(crate) app: Option<String>,

    #[arg(value_enum, value_name = "location")]
    pub(crate) location: Location,

    /// Optional list of path elements to append
    #[arg(value_name = "pathel")]
    pub(crate) files: Option<Vec<String>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Location {
    Home,
    Desktop,
    Downloads,
    Templates,
    Documents,
    Pictures,
    Videos,
    Music,
    Public,
    Config,
    Bin,
    Cache,
    State,
    Data,
    Autostart,
    Fonts,
    Menus,
    Backgrounds,
    Icons,
}
