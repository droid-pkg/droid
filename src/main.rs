mod commands;

use structopt::StructOpt;

#[derive(StructOpt)]
enum Droid {
    #[structopt(aliases = &["i", "-S"])]
    Install { package: String },
}

fn main() {
    match Droid::from_args() {
        Droid::Install { package } => {
            commands::install(package);
        }
    }
}
