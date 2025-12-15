use serde::Deserialize;
use std::sync::Arc;
use toml;
#[derive(Deserialize, Clone, Debug)]
pub struct Paths {
    pub texture_dir: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
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
    pub fn new(path: &str) -> Self {
        let config = Self::load_file(path).unwrap();
        Self {
            config: Arc::new(config),
        }
    }

    fn load_file(path: &str) -> Result<Config, String> {
        let mut file = std::fs::File::open(path).unwrap();
        let mut contents = String::new();
        std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
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
