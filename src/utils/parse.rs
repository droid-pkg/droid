use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallInstructions {
    pub info: Info,
    pub dist: Dist,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
    pub repo_owner: String,
    pub repo_name: String,
    pub license: String,
    pub version_prefix: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dist {
    pub types: Vec<String>,
    pub depends: Vec<String>,
    pub build: Option<Build>,
    pub bin: Option<Bin>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Build {
    pub depends: Vec<String>,
    pub output_file: String,
    pub commands: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bin {
    pub file_name: String,
}

impl InstallInstructions {
    pub fn parse(contents: String) -> Result<InstallInstructions> {
        let parsed_toml: InstallInstructions = toml::from_str(&contents)?;
        Ok(parsed_toml)
    }
}
