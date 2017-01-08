# ithos [![Latest Version][crate-image]][crate-link] [![Build Status][build-image]][build-link] [![Apache 2 licensed][license-image]][license-link]

[crate-image]: https://img.shields.io/crates/v/ithos.svg
[crate-link]: https://crates.io/crates/ithos
[build-image]: https://travis-ci.org/cryptosphere/ithos.svg?branch=master
[build-link]: https://travis-ci.org/cryptosphere/ithos
[license-image]: https://img.shields.io/badge/license-Apache2-blue.svg
[license-link]: https://github.com/cryptosphere/ithos-rb/blob/master/LICENSE

Modern directory services and credential management

## What is ithos?

**ithos** (pronounced ˈēTHōs like "ethos") is a modern directory server
designed to be a master access control system for a fleet of Linux or other
Unix-like servers. The design is inspired by [LDAP], but using
[gRPC]<sup>†</sup> and [JSON] APIs in lieu of the LDAP wire protocol. As
**ithos** is intended for highly secure applications, it's written in the
[Rust] language to ensure safety.

The key differentiating feature of **ithos** over other directory servers is the
use of a cryptographically authenticated append-only log, similar to a
"blockchain", to mediate all changes to the directory. This means every change
is fully auditable and can be attributed to one or more credentials for users
or automated processes who authorized the change.

*<sup>†</sup>NOTE: gRPC support forthcoming*

[LDAP]: https://en.wikipedia.org/wiki/Lightweight_Directory_Access_Protocol
[gRPC]: http://www.grpc.io/
[JSON]: http://www.json.org
[Rust]: https://www.rust-lang.org/

### Is it any good?

[Yes.](http://news.ycombinator.com/item?id=3067434)

### Is it "Production Ready™"?

![DANGER: EXPERIMENTAL](https://raw.github.com/cryptosphere/cryptosphere/master/images/experimental.png)

**ithos** does not yet provide the minimum viable functionality it needs to
be useful. The documentation below covers the current functionality, but
it does not yet explain how to deploy a practical production system.

tl;dr: Not ready yet. Check back later.

## Building

These instructions assume you have a Rust installation. If you haven't yet
installed Rust, please visit https://www.rustup.rs/ for instructions on how to
install Rust.

1. Clone the **ithos** git repository:

```
$ git clone https://github.com/cryptosphere/ithos.git
Cloning into 'ithos'...
```

2. Compile **ithos** with Cargo:

```
$ cargo build --release
```

3. Ensure binary works:

```
$ target/release/ithos -h
ithos v0.1

USAGE:
    ithos [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    db        Creates a new ithos database
    domain    Adds a new domain to an ithos database
    help      Prints this message or the help of the given subcommand(s)
```

## Usage

### Creating a new **ithos** database

1. Create a filesystem directory where the database will live:

```
$ mkdir my_ithos
```

2. Create a new **ithos** database inside the newly created directory:

```
$ target/release/ithos db my_ithos
Creating database at: my_ithos

Database created! Below is the password for the admin user ('manager')
Don't lose it! You will need it to perform administrative actions:

ITHOS-GENPASS-xitak-refuk-lipef-zuxax-48214
```

3. Create an initial domain within your **ithos** database

```
$ target/release/ithos domain example.com --path my_ithos
Creating domain 'example.com' in database at my_ithos
manager's password:
Domain example.com created!
```

## License

Copyright (c) 2016-2017 Tony Arcieri. Distributed under the Apache 2.0 License.
See [LICENSE] file for further details.

[LICENSE]: https://github.com/cryptosphere/ithos/blob/master/LICENSE
