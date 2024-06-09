use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::shells::*;
use clap_mangen::Man;
use std::fs::File;
use std::path::Path;

include!("src/cli.rs");

fn main() {
    let bin_name = "places";
    let outdir = "./support/completions";
    let mut app = Cli::command();

    generate_to(Bash, &mut app, bin_name, &outdir).expect("Failed to generate Bash completions");
    generate_to(Fish, &mut app, bin_name, &outdir).expect("Failed to generate Fish completions");
    generate_to(Zsh, &mut app, bin_name, &outdir).expect("Failed to generate Zsh completions");
    generate_to(Elvish, &mut app, bin_name, &outdir)
        .expect("Failed to generate Elvish completions");

    let file = Path::new("support/manpage").join("places.1");
    let mut file = File::create(file).expect("Failed to generate man page");

    Man::new(app)
        .render(&mut file)
        .expect("Failed to generate man page");
}
