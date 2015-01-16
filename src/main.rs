#![feature(box_syntax)]

extern crate commander;
extern crate semverio;
extern crate downtar;
extern crate semver;

use commander::Commander;

mod downloader;

fn main() {
  //
  // Create our app
  //
  let mut app = Commander::new();

  //
  // Install specified distribution and version
  //
  app.command("install", box |args: Vec<String>| {
    assert!(args.len() >= 2, "Must specify a version");
    assert!(args.len() >= 1, "Must specify a distribution");

    let dist = args[0].to_string();
    let version = args[1].to_string();
    let dest = format!("./versions/{}", dist).to_string();

    let resolved = semverio::resolve(dist.clone(), version).ok().unwrap();

    downloader::download(dist, resolved, dest);
  });

  //
  // Use specified distribution and version
  //
  app.command("use", box |args: Vec<String>| {
    assert!(args.len() >= 2, "Must specify a version");
    assert!(args.len() >= 1, "Must specify a distribution");

    let dist = args[0].to_string();
    let version = args[1].to_string();

    let resolved = semverio::resolve(dist, version).ok().unwrap();

    println!("using... {}", resolved);
  });

  //
  // List install versions
  //
  fn ls (args: Vec<String>) {
    println!("list");
  }
  app.command("list", box ls);
  app.command("ls", box ls);

  //
  // List remote versions
  //
  fn ls_remote (args: Vec<String>) {
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
