use std::path::{ Path };
use std::fs::{ File };

use serde_json;
use manager;

// `Decodable` and `Encodable` json config
#[derive(Serialize, Deserialize)]
pub struct JSEnvConfig {
  pub current_dist: String,
  pub current_version: String,
}

impl JSEnvConfig {
  pub fn load (path: Path) -> JSEnvConfig {
    if path.exists() && path.is_file() {
      // Read file into json object
      let mut f = File::open(&path)?;
      let mut buffer = String::new();

      f.read_to_string(&mut buffer)?;

      serde_json::from_str(&buffer).unwrap_or(JSEnvConfig {
        current_version: "".to_string(),
        current_dist: "".to_string()
      })

    // Make empty config if one does not already exist
    } else {
      JSEnvConfig {
        current_version: "".to_string(),
        current_dist: "".to_string()
      }
    }
  }

  pub fn use_version (dist: String, version: String) {
    let mut config = JSEnvConfig::load(manager::path_to("./config.json"));
    config.current_version = version;
    config.current_dist = dist;
  }
}

// Auto save when config goes out of scope
impl Drop for JSEnvConfig {
  fn drop(&mut self) {
    let mut file = File::open(
      &manager::path_to("./config.json")
    );

    let data = serde_json::to_string_pretty(self).unwrap();

    write!(&mut file, "{}", data);
  }
}
