mod commands;

use anyhow::Result;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Droid {
    #[structopt(aliases = &["-S", "add", "i"])]
    Install { package: String },

    #[structopt(aliases = &["-R", "remove", "rm", "un"])]
    Uninstall { package: String },
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
