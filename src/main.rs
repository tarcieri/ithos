extern crate clap;
use clap::{App, Arg, SubCommand};

extern crate buffoon;
extern crate ring;
extern crate ring_pwhash as pwhash;
extern crate rustc_serialize;
extern crate serde_json;
extern crate time;

mod adapter;
mod block;
mod direntry;
mod entry;
mod error;
mod lmdb_adapter;
mod objectclass;
mod objecthash;
mod op;
mod password;
mod path;
mod server;
mod signature;

use lmdb_adapter::LmdbAdapter;

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
        LmdbAdapter::create_database(std::path::Path::new(path)).unwrap();
    }
}
