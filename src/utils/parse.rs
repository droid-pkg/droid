use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallInstructions {
    pub types: Vec<String>,
    pub depends: Vec<String>,
    pub build: Option<Build>,
    pub bin: Option<Bin>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Build {
    pub build_depends: Vec<String>,
    pub version_prefix: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bin {
    pub file_name: String,
}

impl InstallInstructions {
    pub fn parse(contents: String) -> Result<InstallInstructions> {
        let parsed_yaml: InstallInstructions = serde_yaml::from_str(&contents)?;
        Ok(parsed_yaml)
    }
}
