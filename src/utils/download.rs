use anyhow::Result;
use tokio::{fs, io::AsyncWriteExt};

pub async fn download(url: String, output_file_path: String, file_name: String) -> Result<i32> {
    let client = reqwest::Client::new();

    let mut file_download = client.get(url).send().await?;
    let mut output_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_file_path)
        .await?;

    while let Some(chunk) = file_download.chunk().await? {
        output_file.write_all(&chunk).await?;
    }

    println!("Downloaded");

    Ok(0)
}
