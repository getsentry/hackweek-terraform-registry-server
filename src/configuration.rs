use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub root_module_dir: PathBuf,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}
