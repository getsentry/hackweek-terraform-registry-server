use serde::Deserialize;
use std::path::Path;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub root_module_dir: PathBuf,
    pub host: String,
    pub port: u16,
    pub base_url: String,
}

impl Default for Settings {
    fn default() -> Settings {
      Settings::from_path(Path::new("configuration.yaml"))
    }
}

impl Settings {
  fn from_path(path: &Path) -> Result<Settings> {
        let settings = config::Config::builder()
            .add_source(path.into())
            .build()
            .expect("failed to build configuration");
        settings
            .try_deserialize::<Settings>()
            .expect("failed to deserialize configuration")
}
}
