#![feature(box_syntax)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate serialize;
extern crate commander;
extern crate semverio;
extern crate downtar;
extern crate semver;

// Use commander
use commander::Commander;

// Connect internal modules
mod downloader;
mod commands;
mod manager;
mod config;

fn main() {
  let mut app = Commander::new();

  app.command("install", box commands::install);
  app.command("use", box commands::use_version);
  app.command("list", box commands::list);
  app.command("ls", box commands::list);
  app.command("list-remote", box commands::list_remote);
  app.command("lsr", box commands::list_remote);

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
