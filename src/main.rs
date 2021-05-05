extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;

mod commands;
mod utils;

use anyhow::Result;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Droid {
    /// Installs a package from the repos
    #[structopt(aliases = &["-S", "add", "i"])]
    Install {
        /// The name of the package to install
        package: String,
    },
    /// Uninstalls a package from your computer
    #[structopt(aliases = &["-R", "remove", "rm", "un"])]
    Uninstall {
        /// The name of the packge to uninstall
        package: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    match run().await {
        Ok(res) => std::process::exit(res),
        Err(e) => return Err(e),
    }
}

async fn run() -> Result<i32> {
    match Droid::from_args() {
        Droid::Install { package } => return commands::install(package).await,
        Droid::Uninstall { package } => return commands::uninstall(package).await,
    };
}
