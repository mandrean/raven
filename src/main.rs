extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};
use reqwest::StatusCode;
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

    subcommands(&matches, &repository)
}

fn subcommands(matches: &ArgMatches, repository: &Url) {
    match matches.subcommand() {
        ("checksum", Some(checksum_matches)) => checksum_cmd(repository, checksum_matches),
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(),
    }
}

fn checksum_cmd(repository: &Url, checksum_matches: &ArgMatches) {
    let algorithm = Algorithm::from_str(
        checksum_matches
            .value_of("algorithm")
            .expect("Missing checksum algorithm"),
    )
    .expect("Error parsing Algorithm");

    let coordinates = match MavenCoordinates::parse(
        checksum_matches
            .value_of("Maven coordinates")
            .expect("Missing Maven coordinates"),
    ) {
        Ok(c) => c,
        Err(e) => return eprintln!("{}", e),
    };

    match coordinates.fetch_checksum(repository, algorithm) {
        Ok(checksum) => println!("{}", checksum),
        Err(e) => handler(&coordinates, e),
    };
}

fn handler(coordinates: &MavenCoordinates, e: reqwest::Error) {
    if e.is_http() {
        match e.url() {
            None => eprintln!("No Url given"),
            Some(url) => eprintln!("Problem making request to: {}", url),
        }
    }

    if e.is_client_error() {
        match e.status() {
            Some(StatusCode::NOT_FOUND) => eprintln!("Couldn't find artifact: {}", coordinates),
            _ => return,
        }
    }

    if e.is_serialization() {
        let serde_error = match e.get_ref() {
            None => return,
            Some(err) => err,
        };
        eprintln!("Problem parsing information {}", serde_error);
    }

    if e.is_redirect() {
        eprintln!("Server redirecting too many times or making loop");
    }
}
