#![crate_name = "ithos"]
#![crate_type = "bin"]

extern crate clap;
use clap::{App, Arg, SubCommand};

#[macro_use]
extern crate buffoon;

#[macro_use]
extern crate objecthash;

extern crate ring;
extern crate ring_pwhash as pwhash;
extern crate rustc_serialize;
extern crate serde_json;
extern crate time;

#[cfg(test)]
extern crate tempdir;

mod adapter;
mod algorithm;
mod block;
mod direntry;
mod entry;
mod error;
mod log;
mod metadata;
mod object;
mod op;
mod password;
mod path;
mod proto;
mod server;
mod signature;

use ring::rand;

use error::Error;
use server::Server;

const DEFAULT_ADMIN_USERNAME: &'static str = "manager";

fn main() {
    let version = "v0.1";

    let create_command = SubCommand::with_name("create")
        .about("Creates a new ithos database")
        .arg(Arg::with_name("path")
            .help("Path where the database will be located")
            .index(1)
            .required(true))
        .arg_from_usage("-u, --username=[NAME] 'Username of the admin user (default: manager)'");

    let matches = App::new("ithos")
        .version(version)
        .subcommand(create_command)
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("create") {
        let database_path = matches.value_of("path").unwrap();
        let admin_username = matches.value_of("username").unwrap_or(DEFAULT_ADMIN_USERNAME);

        create(&database_path, &admin_username);
    }
}

fn create(database_path: &str, admin_username: &str) {
    println!("Creating database at: {path}", path = database_path);

    let rng = rand::SystemRandom::new();
    let admin_password = password::generate(&rng);

    match Server::create_database(&std::path::Path::new(database_path),
                                  &rng,
                                  &admin_username,
                                  &admin_password) {
        Ok(_) => {
            println!("\nDatabase created! Below is the password for the admin user: '{admin}'",
                     admin = admin_username);
            println!("Don't lose it! You will need it to perform administrative actions:\n");

            println!("{password}", password = admin_password);
        }
        Err(Error::EntryAlreadyExists) => {
            println!("*** Error: a database already exists at {path}",
                     path = database_path);
        }
        Err(err) => panic!(err),
    }
}
