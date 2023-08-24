use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub root_module_dir: PathBuf,
    pub host: String,
    pub port: u16,
    pub base_url: String,
}

impl Default for Settings {
    fn default() -> Self {
        let settings = config::Config::builder()
            .add_source(config::File::new(
                "configuration.yaml",
                config::FileFormat::Yaml,
            ))
            .build()
            .expect("failed to build configuration");
        settings
            .try_deserialize::<Settings>()
            .expect("failed to deserialize configuration")
    }
}
