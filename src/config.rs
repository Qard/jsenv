use std::io::{File,FileMode,FileAccess};
use std::io::fs::PathExtensions;
use serialize::json::Decoder;
use serialize::Decodable;
use serialize::json;
use manager;

// `Decodable` and `Encodable` json config
#[derive(Decodable, Encodable)]
pub struct JSEnvConfig {
  pub current_dist: String,
  pub current_version: String,
}

impl JSEnvConfig {
  pub fn load (path: Path) -> JSEnvConfig {
    if path.exists() && path.is_file() {
      // Read file into json object
      let mut file = File::open(&path);
      let json_obj = json::from_reader(&mut file).unwrap();

      // Decode json object to struct
      let mut decoder = Decoder::new(json_obj);
      Decodable::decode(&mut decoder)
        .unwrap_or(JSEnvConfig {
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
    let mut file = File::open_mode(
      &manager::path_to("./config.json"),
      FileMode::Truncate,
      FileAccess::Write
    );

    let data = json::as_pretty_json(self)
      .indent(2);

    write!(&mut file, "{}", data);
  }
}
