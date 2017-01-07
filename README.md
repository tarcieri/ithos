# ithos [![Latest Version][crate-image]][crate-link] [![Build Status][build-image]][build-link] [![Apache 2 licensed][license-image]][license-link]

[crate-image]: https://img.shields.io/crates/v/ithos.svg
[crate-link]: https://crates.io/crates/ithos
[build-image]: https://travis-ci.org/cryptosphere/ithos.svg?branch=master
[build-link]: https://travis-ci.org/cryptosphere/ithos
[license-image]: https://img.shields.io/badge/license-Apache2-blue.svg
[license-link]: https://github.com/cryptosphere/ithos-rb/blob/master/LICENSE

Modern directory services and credential management

## What is ithos?

**ithos** (pronounced ˈēTHōs like "ethos") is a modern directory server designed
to be a master access control system for a fleet of Linux or other Unix-like
servers. The design is inspired by [LDAP], but using [gRPC] and [JSON] APIs in
lieu of the LDAP wire protocol. As **ithos** is intended for highly secure
applications, it's written in the [Rust] language to ensure safety.

The key differentiating feature of **ithos** over other directory servers is the
use of a cryptographically authenticated append-only log, similar to a
"blockchain", to mediate all changes to the directory. This means every change
is fully auditable and can be attributed to one or more credentials for users
or automated processes who authorized the change.

[LDAP]: https://en.wikipedia.org/wiki/Lightweight_Directory_Access_Protocol
[gRPC]: http://www.grpc.io/
[JSON]: http://www.json.org
[Rust]: https://www.rust-lang.org/

## Usage

Coming "soon"!

## License

Copyright (c) 2016-2017 Tony Arcieri. Distributed under the Apache 2.0 License.
See [LICENSE] file for further details.

[LICENSE]: https://github.com/cryptosphere/ithos/blob/master/LICENSE
