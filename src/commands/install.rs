use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::env::var;
use std::fs;

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
    let droid_path = format!("{}/.droid", var("HOME").unwrap());
    let droid_bin_path = format!("{}/bin", droid_path);

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));

    fs::create_dir_all(&droid_bin_path)?;

    let releases = client
        .get("https://api.github.com/repos/MrDogeBro/quicknav/releases/latest")
        .headers(headers)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    utils::download(
        format!(
            "https://github.com/mrdogebro/quicknav/releases/download/{}/quicknav",
            releases["tag_name"].as_str().unwrap()
        ),
        format!("{}/quicknav", &droid_bin_path),
        "quicknav".to_string(),
    )
    .await?;

    Ok(0)
}
