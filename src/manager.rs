use std::path::{ Path };
use std::fs;
use std::os;

pub fn path_to(dest: String) -> Path {
    Path::new(dest)
  // let root = os::self_exe_path().unwrap();
  // root.dir_path().join(Path::new(dest))
}

pub fn has_version (dist: String, version: String) -> bool {
  if ! has_distribution(dist.clone()) {
    return false
  }

  path_to(format!("./versions/{}/v{}", dist, version)).exists()
}

pub fn has_distribution (dist: String) -> bool {
  path_to(format!("./versions/{}", dist)).exists()
}

pub fn list_versions (dist: String) -> Vec<String> {
  let base = format!("./versions/{}", dist);
  fs::read_dir(&path_to(base)).unwrap().iter()
    .map(|v| v.filename_str().unwrap().to_string())
    .collect()
}

pub fn list_distributions () -> Vec<String> {
  fs::read_dir(&path_to("./versions")).unwrap().iter()
    .map(|v| v.filename_str().unwrap().to_string())
    .collect()
}

// pub fn use_version (dist: String, version: String) {
//   let base = os::make_absolute(&Path::new("./versions")).unwrap();
//
//   let mut paths = os::getenv_as_bytes("PATH")
//     .map_or(Vec::new(), os::split_paths);
//
//   let mut paths: Vec<&Path> = paths.iter()
//     .filter(|v| !base.is_ancestor_of(*v))
//     .collect();
//
//   let new_path = os::make_absolute(
//     &Path::new(format!("./versions/{}/{}", dist, version))
//   ).unwrap();
//
//   paths.push(&new_path);
//
//   os::setenv("PATH", os::join_paths(paths.as_slice()).unwrap());
// }
