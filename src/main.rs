#![crate_name = "ithos"]
#![crate_type = "bin"]

extern crate clap;
use clap::{App, Arg, SubCommand};

#[macro_use]
extern crate buffoon;

extern crate objecthash;
extern crate ring;
extern crate ring_pwhash as pwhash;
extern crate rustc_serialize;
extern crate serde_json;
extern crate time;

#[cfg(test)]
extern crate tempdir;

macro_rules! objecthash_struct(
    { $hasher:expr, $($key:expr => $value:expr),+ } => {
        {
            let mut digests: Vec<Vec<u8>> = Vec::new();

            $(
                let kd = objecthash::digest(&String::from($key));
                let vd = objecthash::digest(&$value);
                let mut d = Vec::with_capacity(kd.as_ref().len() + vd.as_ref().len());
                d.extend_from_slice(&kd.as_ref());
                d.extend_from_slice(&vd.as_ref());
                digests.push(d);
            )+

            digests.sort();

            $hasher.update(objecthash::types::DICT_TAG);
            for value in &digests {
                $hasher.update(&value);
            }
        }
     };
);

mod adapter;
mod algorithm;
mod block;
mod direntry;
mod entry;
mod error;
mod log;
mod metadata;
mod objectclass;
mod op;
mod password;
mod path;
mod proto;
mod server;
mod signature;

use adapter::lmdb::LmdbAdapter;

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
