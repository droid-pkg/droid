use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::header;
use tokio::{fs, io::AsyncWriteExt};

pub async fn download(url: String, output_file_path: String, file_name: String) -> Result<i32> {
    let client = reqwest::Client::new();

    let file_size: u64 = match client.head(&url).send().await {
        Ok(resp) => resp
            .headers()
            .get(header::CONTENT_LENGTH)
            .and_then(|len| len.to_str().ok())
            .and_then(|len| len.parse().ok())
            .unwrap_or(0),
        Err(e) => return Err(anyhow::anyhow!(e)),
    };

    let progress = ProgressBar::new(file_size);
    let progress_msg = format!("{} {}", "Downloading".green().bold(), file_name);
    progress.set_message(progress_msg);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("    {msg} [{wide_bar}] {bytes}/{total_bytes} ({eta})    ")
            .progress_chars("=>-"),
    );

    let mut file_download = client.get(url).send().await?;
    let mut output_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_file_path)
        .await?;

    while let Some(chunk) = file_download.chunk().await? {
        output_file.write_all(&chunk).await?;
        progress.inc(chunk.len() as u64);
    }

    progress.finish();

    Ok(0)
}
