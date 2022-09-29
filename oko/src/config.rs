use serde::{Deserialize, Serialize};
use std::{error::Error, io, path::Path};

pub const CONFIG_PATH: &str = "./config.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
   pub library: String,
   pub trash: String,
}

impl Config {
   pub fn from_path(path: &str) -> Result<Config, Box<dyn Error>> {
      let path = Path::new(path);
      if path.exists() {
         return Ok(serde_json::from_str(&std::fs::read_to_string(&path)?)?);
      }
      let config = Config {
         library: String::from("~/Pictures/"),
         trash: String::from("~/trash/"),
      };
      let config_str = serde_json::to_string(&config)?;
      std::fs::write(path, config_str)?;
      Ok(config)
   }

   pub fn save(&self) -> Result<(), io::Error> {
      let config_str = serde_json::to_string(self)?;
      std::fs::write(CONFIG_PATH, config_str)?;
      Ok(())
   }
}
