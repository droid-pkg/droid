use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::fs;
use std::os::unix::fs::PermissionsExt;

use crate::utils;

// repos file needs to contain
// ==========================
// - name: str
// - description: str
// - author: str
// - repo_name: str
// - license: str

// install instructions file needs to contain
// ==========================================
// - types: arr (source, git, bin)
// - depends: arr<str>
// source/git only
// - build_depends: arr<str>
// - version_prefix: Opt<str>
// - build func
// - install func
// bin only
// - file name

pub async fn install(package: String) -> Result<i32> {
    let client = reqwest::Client::new();
    let droid_path = format!("/usr/local/droid");
    let droid_bin_path = format!("{}/bin", droid_path);

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));

    fs::create_dir_all(&droid_bin_path)?;

    #[cfg(debug_assertions)]
    let instructions_file = fs::read_to_string("./demo-files/quicknav.yaml")?;
    #[cfg(not(debug_assertions))]
    let instructions_file: String = "types: [bin]\ndepends: []";

    let instructions = utils::InstallInstructions::parse(instructions_file)?;

    let releases = client
        .get("https://api.github.com/repos/MrDogeBro/quicknav/releases/latest")
        .headers(headers)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    if instructions.types.iter().any(|t| t == "bin") {
        install_bin(releases, instructions, droid_bin_path).await?;
    }

    Ok(0)
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
                instructions.author,
                instructions.repo_name,
                releases["tag_name"].as_str().unwrap(),
                bin.file_name
            ),
            file_path.to_string(),
            "quicknav".to_string(),
        )
        .await?;

        let mut perms = fs::metadata(file_path.to_string())?.permissions();
        perms.set_mode(33261);
        fs::set_permissions(file_path, perms)?;
    }

    Ok(0)
}
