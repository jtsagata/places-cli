use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::shells::*;

include!("src/cli.rs");

fn main() {
    let outdir = "./support/completions";
    let mut app = Cli::command();
    let bin_name = "places";

    for &shell in Shell::value_variants() {
        generate_to(shell, &mut app, bin_name, outdir)
            .unwrap_or_else(|_| panic!("Failed to generate {} completions", shell));
    }
}
