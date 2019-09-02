extern crate clap;

use clap::{App, Arg, SubCommand};
use rvn::checksum::Algorithm;
use rvn::MavenCoordinates;
use std::str::FromStr;
use url::Url;

const RVN_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const RVN_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const RVN_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let matches = App::new("raven")
        .version(RVN_VERSION)
        .author(RVN_AUTHORS)
        .about(RVN_DESCRIPTION)
        .subcommand(
            SubCommand::with_name("checksum")
                .about("Prints checksum of Maven artifact")
                .arg(
                    Arg::with_name("Maven coordinates")
                        .help("Maven coordinates of artifact")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("algorithm")
                        .help("cryptographic hash algorithm")
                        .short("a")
                        .long("algo")
                        .alias("algorithm")
                        .possible_values(&Algorithm::variants())
                        .default_value(Algorithm::Sha1.into()),
                ),
        )
        .arg(
            Arg::with_name("repository")
                .help("Maven repository URL")
                .short("r")
                .long("repo")
                .alias("repository")
                .global(true)
                .default_value("https://repo1.maven.org/maven2"),
        )
        .get_matches();

    let repository = match matches.value_of("repository").map(|s| Url::parse(s)) {
        Some(Ok(url)) => url,
        _ => return eprintln!("Error parsing repository URL"),
    };

    match matches.subcommand() {
        ("checksum", Some(checksum_matches)) => {
            let algorithm = Algorithm::from_str(
                checksum_matches
                    .value_of("algorithm")
                    .expect("Missing checksum algorithm"),
            )
            .expect("Error parsing Algorithm");
            let coordinates = checksum_matches.value_of("Maven coordinates").unwrap();
            let checksum = MavenCoordinates::parse(coordinates)
                .unwrap()
                .fetch_checksum(&repository, algorithm)
                .unwrap();
            println!("{}", checksum);
        }
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(),
    }
}
