raven - rvn
===========
A CLI tool for interacting with Maven repositories & artifacts, written in Rust.

[![Build Status](https://travis-ci.org/mandrean/raven.svg?branch=master)](https://travis-ci.org/mandrean/raven)
[![Latest version](https://img.shields.io/crates/v/rvn.svg)](https://crates.io/crates/rvn)
[![Documentation](https://docs.rs/rvn/badge.svg)](https://docs.rs/rvn)
![License](https://img.shields.io/crates/l/rvn.svg)

Install
-------
```sh
$ cargo install rvn
```

Usage
-----
See `rvn --help`.

It currently supports fetching checksums using the `checksum` subcommand followed by the Maven coordinates:

`groupId:artifactId[:packaging[:classifier]]:version`

where `packaging` defaults to `jar` and `classifier` defaults to null: 

```sh
$ rvn checksum io.prometheus:simpleclient_hotspot:0.6.0
sha1:2703b02c4b2abb078de8365f4ef3b7d5e451382d

$ rvn checksum io.prometheus:simpleclient_hotspot:0.6.0 -a md5
md5:13922a158ae99ec67f7bd6ab1853fd93

$ rvn checksum io.prometheus:simpleclient_hotspot:jar:0.6.0
sha1:2703b02c4b2abb078de8365f4ef3b7d5e451382d

$ rvn checksum io.prometheus:simpleclient_hotspot:pom:0.6.0
sha1:c729b1e7de459e9dfe78eb9e4e1dbf47afd96ed9

$ rvn checksum io.prometheus:simpleclient_hotspot:jar:sources:0.6.0
sha1:a53e916d7f422ac34d0fb125dea8c940cf4e15c3
```

Contribute
----------
This project follows [semver], [conventional commits] and semantic releasing using [semantic-rs].

[semver]: https://semver.org/
[conventional commits]: https://www.conventionalcommits.org
[semantic-rs]: https://github.com/semantic-rs/semantic-rs
