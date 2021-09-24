use super::chroot;
use anyhow::Result;
use git2::Repository;

pub async fn build(build_path: String) -> Result<i32> {
    chroot::chroot(build_path).await?;

    Repository::clone(
        format!("https://github.com/{}/{}.git", "MrDogeBro", "quicknav").as_str(),
        "quicknav",
    )?;

    Ok(0)
}
