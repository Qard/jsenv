use std::os::consts;
use std::io::fs;
use std::io;
use downtar;
use manager;

pub fn download (dist: String, version: String, dest: String) -> Result<(), String> {
  // Determine base url from dist name
  let base_url = match dist.as_slice() {
    "node" => "https://nodejs.org/dist",
    "iojs" => "https://iojs.org/dist",
    _ => return Err("Unknown distribution".to_string())
  };

  // Get os and arch from env
  let os = consts::SYSNAME;
  let arch = match consts::ARCH {
    "x86_64" => "x64",
    val => val
  };

  // Construct formatted folder and url strings
  let folder = format!("{}-v{}-{}-{}", dist, version, os, arch);
  let url = format!("{}/v{}/{}.tar.gz", base_url, version, folder);

  // Prepare paths
  let tmp_path = manager::path_to("./tmp");
  let dest_path = manager::path_to(dest);

  // Download and untar to the tmp directory
  downtar::download(url, tmp_path.clone());

  // Move archive contents from tmp to dist/version folder
  for item in fs::readdir(&tmp_path).unwrap().iter() {
    let dir = item.filename_str().unwrap();
    if dir.starts_with(dist.as_slice()) {
      // mkdir -p versions/dist
      let base_path = manager::path_to(dest_path.dirname_str().unwrap());
      fs::mkdir_recursive(&base_path, io::USER_DIR);

      // mv tmp/folder versions/dist/version
      fs::rename(item, &dest_path);

      // TODO: recursive unlink/rmdir
      fs::rmdir(&tmp_path);
      break;
    }
  }

  Ok(())
}
