extern crate clap;

use clap::{App, Arg, SubCommand};
use rvn::{MavenCoordinates};

fn main() {
    let matches = App::new("raven")
        .version("0.1.0")
        .author("Sebastian Mandrean <sebastian.mandrean@gmail.com>")
        .about("A CLI tool for interacting with Maven repositories & artifacts")
        .subcommand(SubCommand::with_name("checksum")
            .about("Prints checksum of Maven artifact")
            .arg(Arg::with_name("Maven coordinates")
                .help("Maven coordinates of artifact")
                .index(1)
                .required(true))
            .arg(Arg::with_name("algorithm")
                .help("cryptographic hash algorithm")
                .short("a")
                .long("algo")
                .alias("algorithm")
                .default_value("sha1"))
        )
        .arg(Arg::with_name("repository")
            .help("Maven repository URL")
            .short("r")
            .long("repo")
            .alias("repository")
            .global(true)
            .default_value("https://repo1.maven.org/maven2"))
        .get_matches();

    let repository = matches.value_of("repository").unwrap();

    match matches.subcommand() {
        ("checksum", Some(checksum_matches)) => {
            let algorithm = checksum_matches.value_of("algorithm").unwrap();
            let coordinates = checksum_matches.value_of("Maven coordinates").unwrap();
            let checksum = MavenCoordinates::parse(coordinates).unwrap().fetch_checksum(repository, algorithm).unwrap();
            println!("{}", checksum);
        },
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(),
    }
}
