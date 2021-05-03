use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::env::var;
use std::process::Command;

pub async fn install(package: String) -> Result<i32> {
    let client = reqwest::Client::new();
    let droid_path = format!("{}/.droid", var("HOME").unwrap());
    let droid_bin_path = format!("{}/bin", droid_path);

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));

    std::fs::create_dir_all(&droid_bin_path)?;

    let releases = client
        .get("https://api.github.com/repos/MrDogeBro/quicknav/releases/latest")
        .headers(headers)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Command::new("curl")
        .arg("-L")
        .arg(format!(
            "https://github.com/mrdogebro/quicknav/releases/download/{}/quicknav",
            releases["tag_name"].as_str().unwrap()
        ))
        .arg("-o")
        .arg(format!("{}/quicknav", &droid_bin_path))
        .output()?;

    Ok(0)
}
