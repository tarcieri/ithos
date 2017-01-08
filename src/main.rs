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
extern crate rpassword;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;
extern crate time;

#[cfg(test)]
extern crate tempdir;

mod adapter;
mod algorithm;
mod block;
mod direntry;
mod encryption;
mod entry;
mod error;
mod metadata;
mod object;
mod op;
mod password;
mod path;
mod proto;
mod server;
mod signature;
mod timestamp;
pub mod witness;

use algorithm::CipherSuite;
use adapter::lmdb::LmdbAdapter;
use encryption::AES256GCM_KEY_SIZE;
use error::Error;
use path::PathBuf;
use ring::rand;
use server::Server;

const DEFAULT_ADMIN_USERNAME: &'static str = "manager";

fn main() {
    let version = "v0.1";

    let db_create_command = SubCommand::with_name("db")
        .about("Creates a new ithos database")
        .arg(Arg::with_name("path")
            .help("Path where the database will be located")
            .index(1)
            .required(true))
        .arg_from_usage("-u, --username=[NAME] 'Username of the admin user (default: manager)'");

    let domain_add_command = SubCommand::with_name("domain")
        .about("Adds a new domain to an ithos database")
        .arg(Arg::with_name("domain")
            .help("Domain name to add to the database")
            .index(1)
            .required(true))
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .help("Path to the ithos database")
            .takes_value(true)
            .required(true))
        .arg_from_usage("-u, --username=[NAME] 'Username to authenticate with'");

    let matches = App::new("ithos")
        .version(version)
        .subcommand(db_create_command)
        .subcommand(domain_add_command)
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("db") {
        let db_path = matches.value_of("path").unwrap();
        let admin_username = matches.value_of("username").unwrap_or(DEFAULT_ADMIN_USERNAME);

        db_create(db_path, admin_username);
    } else if let Some(matches) = matches.subcommand_matches("domain") {
        let domain = matches.value_of("domain").unwrap();
        let db_path = matches.value_of("path").unwrap();
        let username = matches.value_of("username").unwrap_or(DEFAULT_ADMIN_USERNAME);

        domain_add(db_path, username, domain);
    }
}

fn db_create(database_path: &str, admin_username: &str) {
    println!("Creating database at: {path}", path = database_path);

    let rng = rand::SystemRandom::new();
    let admin_password = password::generate(&rng);

    match Server::<LmdbAdapter>::create_database(std::path::Path::new(database_path),
                                                 &rng,
                                                 CipherSuite::Ed25519Aes256GcmSha256,
                                                 admin_username,
                                                 &admin_password) {
        Ok(_) => {
            println!("\nDatabase created! Below is the password for the admin user ('{admin}')",
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

fn domain_add(database_path: &str, admin_username: &str, domain_name: &str) {
    println!("Creating domain '{domain}' in database at {path}",
             path = database_path,
             domain = domain_name);

    let server = Server::<LmdbAdapter>::open_database(std::path::Path::new(database_path))
        .unwrap_or_else(|err| {
            panic!("*** Error: couldn't open database at {path}: {err}",
                   path = database_path,
                   err = err);
        });

    let mut keypair_path = PathBuf::new();
    keypair_path.push("global");
    keypair_path.push("users");
    keypair_path.push(&admin_username);
    keypair_path.push("keys");
    keypair_path.push("signing");

    let admin_credential = server.find_credential(keypair_path.as_ref()).unwrap_or_else(|err| {
        panic!("*** Error: couldn't find admin keypair for {username}: {err}",
               username = admin_username,
               err = err);
    });

    let admin_password = password::prompt(&format!("{}'s password: ", admin_username)).unwrap();

    let salt = match admin_credential.salt {
        Some(ref s) => s,
        None => panic!("salt missing!"),
    };

    let mut admin_symmetric_key = [0u8; AES256GCM_KEY_SIZE];
    password::derive(password::PasswordAlgorithm::SCRYPT,
                     salt,
                     &admin_password,
                     &mut admin_symmetric_key);

    let admin_keypair = admin_credential.unseal_signature_keypair(&admin_symmetric_key)
        .unwrap_or_else(|err| {
            panic!("*** Error: couldn't decrypt admin keypair: {} (wrong password?)",
                   err)
        });

    let comment = format!("Creating {domain} domain", domain = domain_name);

    // TODO: description support
    match server.add_domain(&admin_keypair, domain_name, None, &comment) {
        Ok(_) => {
            println!("Domain {domain} created!", domain = domain_name);
        }
        Err(err) => {
            panic!("*** Error: couldn't create domain {domain}: #{err}",
                   domain = domain_name,
                   err = err);
        }
    };
}
