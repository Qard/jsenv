#[macro_use]
extern crate serde_derive;
extern crate serde_json;

// extern crate semverio;
// extern crate downtar;
// extern crate semver;

#[macro_use]
extern crate clap;

use std::path::{ Path };
use clap::{ Arg, App, SubCommand };

// Connect internal modules
// mod downloader;
// mod commands;
mod manager;
mod config;

use config::{ JSEnvConfig };

fn main() {
    let yaml = load_yaml!("interface.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config_path = matches.value_of("config").unwrap_or("~/.jsenv");
    println!("Value for config: {}", config_path);

    let config = JSEnvConfig::load(Path::new(&config_path));
    println!("config is: {:?}", config);

    if let Some(sub_matches) = matches.subcommand_matches("install") {
        println!("install not yet implemented...");
    }

  // let mut app = Commander::new();
  //
  // app.command("install", box commands::install);
  // app.command("use", box commands::use_version);
  // app.command("list", box commands::list);
  // app.command("ls", box commands::list);
  // app.command("list-remote", box commands::list_remote);
  // app.command("lsr", box commands::list_remote);
  //
  // match app.run() {
  //   Ok(_) => {},
  //   Err(_) => {
  //     println!("Available commands:");
  //     let commands = app.commands();
  //     for cmd in commands.iter() {
  //       println!("  {}", cmd);
  //     }
  //   }
  // };
}
