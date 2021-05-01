mod commands;

use structopt::StructOpt;

#[derive(StructOpt)]
enum Droid {
    #[structopt(aliases = &["-S", "add", "i"])]
    Install { package: String },

    #[structopt(aliases = &["-R", "remove", "rm", "un"])]
    Uninstall { package: String },
}

fn main() {
    match Droid::from_args() {
        Droid::Install { package } => {
            commands::install(package);
        }
        Droid::Uninstall { package } => {
            commands::uninstall(package);
        }
    }
}
