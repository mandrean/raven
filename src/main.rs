extern crate clap;
extern crate regex;
extern crate reqwest;

use clap::{App, Arg, SubCommand};
use regex::Regex;
use reqwest::Error;

fn main() {
    let matches = App::new("raven")
        .version("0.0.0")
        .author("Sebastian Mandrean <sebastian.mandrean@gmail.com>")
        .about("A CLI tool for interacting with Maven repositories & artifacts")
        .subcommand(SubCommand::with_name("checksum")
            .about("Print checksum of a Maven artifact")
            .arg(Arg::with_name("maven_coordinate")
                .help("the Maven coordinates of the artifact")
                .index(1)
                .required(true)))
        .get_matches();

    match matches.subcommand() {
        ("checksum", Some(checksum_matches)) => {
            println!("{}",
                     fetch_checksum(
                         parse_maven_coordinates(
                             checksum_matches.value_of("maven_coordinate").unwrap()
                         ).unwrap()
                     ).unwrap()
            );
        },
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct MavenCoordinates<'a> {
    group_id: &'a str,
    artifact_id: &'a str,
    packaging: Option<&'a str>,
    classifier: Option<&'a str>,
    version: &'a str,
}

fn fetch_checksum<'a>(maven_coordinates: MavenCoordinates) -> Result<String, Error> {
    let group_id_formatted = str::replace(&maven_coordinates.group_id, ".", "/");
    let request_url = format!("https://repo1.maven.org/maven2/{group_id}/{artifact_id}/{version}/{artifact_id}-{version}{classifier}.{packaging}.sha1",
                              group_id = &group_id_formatted,
                              artifact_id = &maven_coordinates.artifact_id,
                              version = &maven_coordinates.version,
                              classifier = &maven_coordinates.classifier.map(|c| format!("-{}", c)).unwrap_or("".to_string()),
                              packaging = &maven_coordinates.packaging.unwrap_or("jar"));

    let mut response = reqwest::get(&request_url)?;

    let checksum = response.text()?;
    Ok(checksum)
}

fn parse_maven_coordinates(maven_coordinates: &str) -> Result<MavenCoordinates, Error> {
    // Parse Maven coordinates into named capture groups, with optional packaging OR packaging+classifier
    let regexp = Regex::new(r"^(?P<groupId>[\w.]+):(?P<artifactId>[\w.\-]+)(?:(?::(?P<packaging>[\w]+))(?::(?P<classifier>[\w]+)?)?)?:(?P<version>[\w.\-]+)$").unwrap();
    let matches = regexp.captures(maven_coordinates).unwrap();

    Ok(MavenCoordinates {
        group_id: &matches.name("groupId").unwrap().as_str(),
        artifact_id: matches.name("artifactId").unwrap().as_str(),
        packaging: matches.name("packaging").map(|m| m.as_str()),
        classifier: matches.name("classifier").map(|m| m.as_str()),
        version: &matches.name("version").unwrap().as_str(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parses_three_component_maven_coordinate() {
        let provided = "com.fasterxml.jackson.core:jackson-annotations:2.9.9";
        let expected = MavenCoordinates {
            group_id: "com.fasterxml.jackson.core",
            artifact_id: "jackson-annotations",
            packaging: None,
            classifier: None,
            version: "2.9.9",
        };
        assert_eq!(parse_maven_coordinates(provided).unwrap(), expected);
    }

    #[test]
    fn test_parses_four_component_maven_coordinate() {
        let provided = "com.fasterxml.jackson.core:jackson-annotations:pom:2.9.9";
        let expected = MavenCoordinates {
            group_id: "com.fasterxml.jackson.core",
            artifact_id: "jackson-annotations",
            packaging: Some("pom"),
            classifier: None,
            version: "2.9.9",
        };
        assert_eq!(parse_maven_coordinates(provided).unwrap(), expected);
    }

    #[test]
    fn test_parses_five_component_maven_coordinate() {
        let provided = "com.fasterxml.jackson.core:jackson-annotations:jar:sources:2.9.9";
        let expected = MavenCoordinates {
            group_id: "com.fasterxml.jackson.core",
            artifact_id: "jackson-annotations",
            packaging: Some("jar"),
            classifier: Some("sources"),
            version: "2.9.9",
        };
        assert_eq!(parse_maven_coordinates(provided).unwrap(), expected);
    }

    #[test]
    fn test_fetches_correct_checksum() {
        let coordinates = "com.fasterxml.jackson.core:jackson-annotations:jar:sources:2.9.9";
        let checksum = "4ac77aa5799fcf00a9cde00cd7da4d08bdc038ff";

        assert_eq!(fetch_checksum(parse_maven_coordinates(coordinates).unwrap()).unwrap(), checksum);
    }
}
