use super::chroot;
use anyhow::Result;
use git2::Repository;

use crate::commands;
use crate::utils;

pub async fn build(
    build_path: String,
    client: reqwest::Client,
    instructions: utils::InstallInstructions,
) -> Result<i32> {
    chroot::chroot(build_path).await?;

    for pkg in instructions.dist.build.unwrap().depends {
        let depend_instructions =
            commands::install::get_instructions(client.to_owned(), pkg).await?;
        let instructs = utils::InstallInstructions::parse(depend_instructions.data);

        if depend_instructions.official {
            // commands::install::install_bin(instructions);
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

    Ok(0)
}
