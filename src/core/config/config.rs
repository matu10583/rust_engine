use serde::Deserialize;
use std::path::Path;
use std::sync::Arc;
use toml;
#[derive(Deserialize, Clone, Debug, Default)]
pub struct Paths {
    pub texture_dir: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Config {
    pub paths: Option<Paths>,
}

impl Config {
    pub fn get_texture_dir(&self) -> Option<String> {
        self.paths.as_ref().unwrap().texture_dir.clone()
    }
}

pub struct ConfigContainer {
    config: Arc<Config>,
}

impl ConfigContainer {
    pub fn empty() -> Self {
        Self::from_config(Config::default())
    }

    pub fn from_config(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, String> {
        Self::load_file(path).map(Self::from_config)
    }

    fn load_file(path: impl AsRef<Path>) -> Result<Config, String> {
        let mut file = std::fs::File::open(path).map_err(|e| e.to_string())?;
        let mut contents = String::new();
        std::io::Read::read_to_string(&mut file, &mut contents).map_err(|e| e.to_string())?;
        let config: Result<Config, toml::de::Error> = toml::from_str(&contents);

        match config {
            Ok(config) => Ok(config),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn get_config(&self) -> Arc<Config> {
        self.config.clone()
    }
}
