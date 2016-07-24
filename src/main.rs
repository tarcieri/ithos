use std::path::Path;

extern crate clap;
use clap::{App, Arg, SubCommand};

extern crate ring;

mod log;
mod lmdb;
mod objecthash;
mod password;
mod server;
mod signature;

use lmdb::Adapter;

fn main() {
    let version = "v0.1";

    let create_command = SubCommand::with_name("create")
        .about("Creates a new ithos database")
        .arg(Arg::with_name("path")
            .help("Path where the database will be located")
            .index(1)
            .required(true));

    let matches = App::new("ithos")
        .version(version)
        .subcommand(create_command)
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("create") {
        let path = matches.value_of("path").unwrap();

        println!("Creating database at: {}", path);
        Adapter::create_database(Path::new(path)).unwrap();
    }
}
