use std::os::consts;
use downtar;

pub fn download (dist: String, version: String, dest: String) -> Result<(), String> {
  let base_url = match dist.as_slice() {
    "node" => "https://nodejs.org/dist",
    "iojs" => "https://iojs.org/dist",
    _ => return Err("Unknown distribution".to_string())
  };

  let os = consts::SYSNAME;
  let arch = match consts::ARCH {
    "x86_64" => "x64",
    val => val
  };

  let folder = format!("{}-v{}-{}-{}", dist, version, os, arch);
  let url = format!("{}/v{}/{}.tar.gz", base_url, version, folder);

  downtar::download(url, dest);
  Ok(())
}
