use clap::Parser;
use directories::{BaseDirs, ProjectDirs, UserDirs};

#[derive(Parser, Debug)]
#[clap(author, version, about = "Get XDG directories and files")]
struct Args {
    #[arg(index = 1)]
    program: String,

    #[arg(index = 2)]
    location: Option<String>,

    #[arg(index = 3)]
    filename: Option<String>,
}

fn main() {
    let args = Args::parse();
    let project = args.program;
    let directory = args.location;
    let filename = args.filename;
    match project.as_str() {
        "xdg" | "dir" => action_xdg(&directory, &filename),
        "home" | "shell" => action_home(&directory, &filename),
        _ => action_program(&project, &directory, &filename),
    }
}

fn action_program(program: &String, directory: &Option<String>, filename: &Option<String>) {
    todo!()
}

fn action_home(directory: &Option<String>, filename: &Option<String>) {
    todo!()
}

fn action_xdg(location: &Option<String>, filename: &Option<String>) {
    let user_dirs = UserDirs::new().unwrap();

    if let Some(location) = location {
        let loc = location.to_lowercase();
        let loc = loc.trim_end_matches('s');
        let path = match loc {
            "audio" => user_dirs.audio_dir(),
            "desktop" => user_dirs.desktop_dir(),
            "document" => user_dirs.document_dir(),
            "download" => user_dirs.download_dir(),
            "font" => user_dirs.font_dir(),
            "home" => Some(user_dirs.home_dir()),
            "picture" => user_dirs.picture_dir(),
            "public" => user_dirs.public_dir(),
            "template" => user_dirs.template_dir(),
            _ => None,
        };

        match path {
            Some(path) => {
                println!("{}", path.display())
            }
            None => {
                eprintln!("invalid location {location}")
            }
        }
    } else {
        println!("TODO: show all xdg data")
    }
}
