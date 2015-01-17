use serialize::json;
use std::io::{File,FileMode,FileAccess};

// Automatically generate `Decodable` and `Encodable` trait implementations
#[derive(Decodable, Encodable)]
pub struct JSEnvConfigData {
  pub current_dist: String,
  pub current_version: String,
}

pub struct JSEnvConfig {
  path: Path,
  pub data: JSEnvConfigData
}

impl JSEnvConfig {
  pub fn load(path: String) -> JSEnvConfig {
    let config_path = Path::new(path);

    let data = File::open(&config_path)
      .read_to_string()
      .unwrap();

    let mut config: JSEnvConfigData = json::decode(data.as_slice())
      .unwrap_or(JSEnvConfigData {
        current_version: "".to_string(),
        current_dist: "".to_string()
      });

    JSEnvConfig {
      path: config_path,
      data: config
    }
  }

  pub fn use_version (dist: String, version: String) {
    let mut config = JSEnvConfig::load("./config.json".to_string());
    config.data.current_version = version;
    config.data.current_dist = dist;
  }
}

impl Drop for JSEnvConfig {
  fn drop(&mut self) {
    let mut file = File::open_mode(
      &self.path,
      FileMode::Truncate,
      FileAccess::Write
    );

    let data = json::as_pretty_json(&self.data)
      .indent(2);

    write!(&mut file, "{}", data);
  }
}
