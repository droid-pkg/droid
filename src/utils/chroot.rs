use anyhow::Result;
use glob::glob;
use std::fs;
use std::os::unix::fs as ufs;
use std::process::Command;

pub async fn chroot(path: String) -> Result<()> {
    fs::create_dir_all(&path)?;

    if cfg!(target_os = "macos") {
        macos_chroot(path.to_owned()).await?;
    } else {
        unix_chroot(path.to_owned()).await?;
    }

    ufs::chroot(path)?;
    std::env::set_current_dir("/")?;

    Ok(())
}

async fn macos_chroot(path: String) -> Result<()> {
    let copy: Vec<&str> = vec!["/bin/**/*", "/usr/lib/dyld", "/usr/lib/system/**/*"];
    let mut files: Vec<String> = vec![];
    let mut depends: Vec<String> = vec![];

    for file in copy {
        for p in glob(file)?.filter_map(std::result::Result::ok) {
            if !p.is_dir() {
                files.push(p.display().to_string());
            }
        }
    }

    depends.extend(macos_otool(files.to_owned()).await?);
    depends.sort();
    depends.dedup();

    depends.extend(macos_otool(depends.to_owned()).await?);
    depends.sort();
    depends.dedup();

    println!("{:#?}", depends);

    setup_chroot(
        path,
        files,
        depends,
        Some(vec![
            "Users/Shared",
            "var/folders",
            "Applications/Utilities",
            "usr/lib/system/introspection",
        ]),
    )
    .await?;

    Ok(())
}

async fn unix_chroot(path: String) -> Result<()> {
    setup_chroot(path, vec![], vec![], Some(vec!["home"])).await?;

    Ok(())
}

async fn macos_otool(files: Vec<String>) -> Result<Vec<String>> {
    let mut depends: Vec<String> = vec![];
    let otool = Command::new("otool").arg("-L").args(&files).output()?;

    for depend in String::from_utf8_lossy(&otool.stdout).lines() {
        if depend.ends_with(":") {
            continue;
        }

        depends.push(
            depend
                .split_whitespace()
                .collect::<Vec<&str>>()
                .get(0)
                .unwrap()
                .to_string(),
        );
    }

    Ok(depends)
}

async fn setup_chroot(
    path: String,
    files: Vec<String>,
    depends: Vec<String>,
    addtional_dirs: Option<Vec<&str>>,
) -> Result<()> {
    let mut dirs = vec![
        "bin",
        "dev",
        "etc",
        "sbin",
        "tmp",
        "usr/bin",
        "usr/include",
        "usr/lib",
        "usr/libexec",
        "usr/local",
        "usr/sbin",
        "usr/share",
        "var/db",
        "var/log",
        "var/run",
        "var/tmp",
    ];

    if let Some(addtional_dirs) = addtional_dirs {
        dirs.extend(addtional_dirs.iter());
    }

    for dir in dirs {
        fs::create_dir_all(format!("{}/{}", path, dir))?;
    }

    for file in files {
        fs::copy(&file, format!("{}{}", path, file))?;
    }

    for depend in depends {
        fs::copy(&depend, format!("{}{}", path, depend))?;
    }

    Ok(())
}
