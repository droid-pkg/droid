use anyhow::{anyhow, Result};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use uuid::Uuid;

use crate::utils;

pub struct RetreviedInstructions {
    official: bool,
    data: String,
}

pub async fn install(package: String) -> Result<i32> {
    let client = reqwest::Client::new();
    let droid_path = format!("/usr/local/droid");
    let droid_bin_path = format!("{}/bin", droid_path);
    let droid_temp_path = format!("{}/temp", droid_path);

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));

    fs::create_dir_all(&droid_bin_path)?;
    fs::create_dir_all(&droid_temp_path)?;

    // use in prod and when testing with files from repos
    // let instructions_file = get_instructions(client.clone(), package).await?;
    // use when wanting to test with a local file
    let instructions_file = RetreviedInstructions {
        official: false,
        data: fs::read_to_string("./demo-files/quicknav.yaml")?,
    };

    let instructions = utils::InstallInstructions::parse(instructions_file.data)?;

    let releases = client
        .get(format!(
            "https://api.github.com/repos/{}/{}/releases/{}",
            instructions.repo_name, instructions.repo_name, "latest"
        ))
        .headers(headers)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    if instructions_file.official {
        if instructions.types.iter().any(|t| t == "bin") {
            install_bin(releases, instructions, droid_bin_path).await?;
        }
    } else {
        // if instructions.types.iter().any(|t| t == "source") {
        // install_bin(releases, instructions, droid_bin_path).await?;
        // }
        let chroot_path = format!("{}/{}", droid_temp_path, Uuid::new_v4());

        utils::build(chroot_path, client).await?;
    }

    Ok(0)
}

pub async fn get_instructions(
    client: reqwest::Client,
    package: String,
) -> Result<RetreviedInstructions> {
    let pkg_path = format!("{}/{}", package[1..].to_string(), package[2..3].to_string());

    let official_repo_file = client
        .get(format!(
        "https://raw.githubusercontent.com/MrDogeBro/droid-repos/HEAD/official/{pkg_path}/{pkg}.yaml",
        pkg_path = pkg_path,
        pkg = package
    ))
        .send()
        .await?
        .text()
        .await?;

    if !(official_repo_file == "404: Not Found") {
        return Ok(RetreviedInstructions {
            official: true,
            data: official_repo_file,
        });
    }

    let user_repo_file = client
        .get(format!(
            "https://raw.githubusercontent.com/MrDogeBro/droid-repos/HEAD/user/{pkg_path}/{pkg}.yaml",
            pkg_path = pkg_path,
            pkg = package
        ))
        .send()
        .await?
        .text()
        .await?;

    if !(user_repo_file == "404: Not Found") {
        return Ok(RetreviedInstructions {
            official: false,
            data: user_repo_file,
        });
    }

    Err(anyhow!("Unable to find a package matching the given name."))
}

async fn install_bin(
    releases: serde_json::Value,
    instructions: utils::InstallInstructions,
    droid_bin_path: String,
) -> Result<i32> {
    let bin = instructions.bin;

    if let Some(bin) = bin {
        let file_path = format!("{}/{}", &droid_bin_path, bin.file_name);

        utils::download(
            format!(
                "https://github.com/{}/{}/releases/download/{}/{}",
                instructions.repo_owner,
                instructions.repo_name,
                releases["tag_name"].as_str().unwrap(),
                bin.file_name
            ),
            file_path.to_string(),
            bin.file_name,
        )
        .await?;

        let mut perms = fs::metadata(file_path.to_string())?.permissions();
        perms.set_mode(33261);
        fs::set_permissions(file_path, perms)?;
    }

    Ok(0)
}
