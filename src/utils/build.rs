use super::chroot;
use anyhow::Result;
use git2::Repository;
use std::process::Command;

use crate::commands;
use crate::utils;

pub async fn build(
    build_path: String,
    client: reqwest::Client,
    instructions: utils::InstallInstructions,
    releases: serde_json::Value,
    droid_bin_path: String,
) -> Result<i32> {
    chroot::chroot(build_path).await?;

    for pkg in instructions.to_owned().dist.build.unwrap().depends {
        let depend_instructions =
            commands::install::get_instructions(client.to_owned(), pkg).await?;
        let instructs = utils::InstallInstructions::parse(depend_instructions.data)?;

        if depend_instructions.official {
            commands::install::install_bin(
                releases.to_owned(),
                instructs,
                droid_bin_path.to_owned(),
            )
            .await?;
        } else {
        }
    }

    Repository::clone(
        format!(
            "https://github.com/{}/{}.git",
            instructions.info.repo_owner, instructions.info.repo_name
        )
        .as_str(),
        instructions.info.repo_name,
    )?;

    for cmd in instructions.dist.build.unwrap().commands {
        let mut args: Vec<&str> = cmd.split(" ").collect();
        let base = args[0];
        args.remove(0);

        // let out = Command::new(base).args(args).output()?;
        let out = Command::new("curl").output()?;
        println!("{:?}", out.stdout);
    }

    Ok(0)
}
