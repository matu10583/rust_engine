use serde::Deserialize;
use std::path::{Path, PathBuf};
use thiserror::Error;
use toml;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config file: {path}")]
    ReadFile {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("failed to parse config file: {path}")]
    ParseToml {
        path: PathBuf,
        source: toml::de::Error,
    },
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Paths {
    pub texture_dir: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct TextureConfig {
    pub texture_dir: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Config {
    pub paths: Option<Paths>,
}

impl Config {
    pub fn texture_config(&self) -> TextureConfig {
        TextureConfig {
            texture_dir: self
                .paths
                .as_ref()
                .and_then(|paths| paths.texture_dir.clone()),
        }
    }
}

pub struct ConfigContainer {
    config: Config,
}

impl ConfigContainer {
    pub fn empty() -> Self {
        Self::from_config(Config::default())
    }

    pub fn from_config(config: Config) -> Self {
        Self { config }
    }

    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        Self::load_file(path).map(Self::from_config)
    }

    fn load_file(path: impl AsRef<Path>) -> Result<Config, ConfigError> {
        let path = path.as_ref();
        let mut file = std::fs::File::open(path).map_err(|source| ConfigError::ReadFile {
            path: path.to_path_buf(),
            source,
        })?;
        let mut contents = String::new();
        std::io::Read::read_to_string(&mut file, &mut contents).map_err(|source| {
            ConfigError::ReadFile {
                path: path.to_path_buf(),
                source,
            }
        })?;
        toml::from_str(&contents).map_err(|source| ConfigError::ParseToml {
            path: path.to_path_buf(),
            source,
        })
    }

    pub fn get_config(&self) -> Config {
        self.config.clone()
    }
}
