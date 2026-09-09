use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub droid_path: String,
    pub droid_bin_path: String,
    pub droid_temp_path: String,
}

impl Config {
    pub fn load() -> Result<Config> {
        let droid_path = format!("/usr/local/droid");

        Ok(Config {
            droid_path: droid_path.to_owned(),
            droid_bin_path: format!("{}/bin", &droid_path),
            droid_temp_path: format!("{}/temp", droid_path),
        })
    }
}
