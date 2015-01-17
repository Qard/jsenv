#![feature(box_syntax)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate serialize;
extern crate commander;
extern crate semverio;
extern crate downtar;
extern crate semver;

use commander::Commander;
use config::JSEnvConfig;
use semver::{Version,VersionReq};

mod downloader;
mod manager;
mod config;

fn main() {
  //
  // Create our app
  //
  let mut app = Commander::new();

  //
  // Install specified distribution and version
  //
  app.command("install", box |args: Vec<String>| {
    // Expect distribution and version arguments
    assert!(args.len() >= 2, "Must specify a version");
    assert!(args.len() >= 1, "Must specify a distribution");

    let dist = args[0].to_string();
    let version = args[1].to_string();

    // Resolve version fragment via semver.io
    let resolved = semverio::resolve(dist.clone(), version).ok().unwrap();

    // Download matching tar and untar it in the versions folder
    // TODO:
    // - Fix downtar to move from tmp folder to versions folder
    // - Check for prior existence of a matching version!
    let dest = format!("./versions/{}/{}", dist, resolved).to_string();
    match downloader::download(dist.clone(), resolved.clone(), dest) {
      Ok(_) => {
        // Set the new version to be used
        JSEnvConfig::use_version(dist.clone(), resolved.clone());
        println!("Installed {} {}", dist, resolved)
      },
      Err(_) => println!("Failed to install {} {}", dist, resolved)
    };
  });

  //
  // Use specified distribution and version
  //
  app.command("use", box |args: Vec<String>| {
    // Expect distribution and version arguments
    assert!(args.len() >= 2, "Must specify a version");
    assert!(args.len() >= 1, "Must specify a distribution");

    let dist = args[0].to_string();
    let version = args[1].to_string();

    // Resolve best match of locally installed versions
    // TODO: Write this, semverio thing is just a sample for now...
    // let resolved = semverio::resolve(dist.clone(), version).ok().unwrap();

    // Verify existence of version of this distro
    if ! manager::has_distribution(dist.clone()) {
      println!("No versions for {} distribution", dist);
      return;
    }

    // Create matcher to scope to semver matches
    let matcher = VersionReq::parse(version.as_slice()).unwrap();

    // Map and filter version strings to semver version instances
    let mut matches: Vec<Version> = manager::list_versions(dist.clone()).iter()
      // Trim "v" prefix and convert to Version instance
      .map(|v| Version::parse(&v[1..v.len()]).unwrap())
      // Filter to only matching instances
      .filter(|v| matcher.matches(v))
      .collect();

    // Sort versions, oldest to newest
    matches.sort();

    // Grab latest version
    let resolved = match matches.last() {
      Some(version) => version.to_string(),
      None => {
        println!("No versions for {} matching {}", dist, version);
        return;
      }
    };

    // Build name string from dist/version
    let fullname = format!("{} {}", dist, format!("v{}", resolved));

    // Verify existence of version of this distro
    if ! manager::has_version(dist.clone(), resolved.clone()) {
      println!("Could not find {}", fullname);
      return;
    }

    // Set the matching version to be used
    JSEnvConfig::use_version(dist, resolved.clone());
    println!("Using {}", fullname);
  });

  //
  // List install versions
  //
  fn ls (args: Vec<String>) {
    // TODO: Allow optional dist argument to scope search
    println!("Installed versions:");
    let distros = manager::list_distributions();
    for distro in distros.iter() {
      println!("  {}:", distro.clone());
      let versions = manager::list_versions(distro.clone());
      for version in versions.iter() {
        println!("    - {}", version);
      }
      println!("");
    }
  }
  app.command("list", box ls);
  app.command("ls", box ls);

  //
  // List remote versions
  //
  fn ls_remote (args: Vec<String>) {
    // TODO: Make dist scoping optional
    assert!(args.len() == 1, "Must specify a distribution");
    let dist = args[0].to_string();
    let versions = semverio::versions(dist).ok().unwrap();
    for version in versions.iter() {
      println!("{}", version);
    }
  }
  app.command("list-remote", box ls_remote);
  app.command("lsr", box ls_remote);

  //
  // Execute command runner
  //
  match app.run() {
    Ok(_) => {},
    Err(_) => {
      println!("Available commands:");
      let commands = app.commands();
      for cmd in commands.iter() {
        println!("  {}", cmd);
      }
    }
  };
}
