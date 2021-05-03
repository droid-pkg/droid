use anyhow::Result;

pub fn download(url: String, output_file: String) -> Result<i32> {
    Command::new("curl")
        .arg("-L")
        .arg(url)
        .arg("-o")
        .arg(output_file)
        .output()?;
}
