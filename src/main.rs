mod cli;

use clap::Parser;
use cli::{Cli, Location};
use directories::{BaseDirs, ProjectDirs, UserDirs};
use std::path::{Path, PathBuf};

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
        // It is expected to always get something, so just called unwrap() until something bad reported
        Location::Home => as_string(user_dirs.home_dir()),
        Location::Desktop => as_string(&user_dirs.desktop_dir().unwrap().join(app)),
        Location::Downloads => as_string(&user_dirs.download_dir().unwrap().join(app)),
        Location::Templates => as_string(&user_dirs.template_dir().unwrap().join(app)),
        Location::Documents => as_string(&user_dirs.document_dir().unwrap().join(app)),
        Location::Pictures => as_string(&user_dirs.picture_dir().unwrap().join(app)),
        Location::Videos => as_string(&user_dirs.video_dir().unwrap().join(app)),
        Location::Music => as_string(&user_dirs.audio_dir().unwrap().join(app)),
        Location::Public => as_string(&user_dirs.public_dir().unwrap().join(app)),
        Location::Config => as_string(app_dirs.config_dir()),
        Location::Bin => as_string(&base_dirs.executable_dir().unwrap().join(app)),
        Location::Cache => as_string(app_dirs.cache_dir()),
        Location::Data => as_string(app_dirs.data_dir()),
        Location::State => as_string(&base_dirs.state_dir().unwrap().join(app)),
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
        // It is expected to always get something, so just called unwrap() until something bad reported
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
