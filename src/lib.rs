extern crate regex;
extern crate reqwest;
extern crate url;

use crate::checksum::Algorithm;
use core::fmt;
use log::debug;
use regex::Regex;
use std::string::ToString;
use url::Url;

pub mod checksum;

/// A parsed Maven coordinates record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MavenCoordinates<'a> {
    pub group_id: &'a str,
    pub artifact_id: &'a str,
    pub packaging: Option<&'a str>,
    pub classifier: Option<&'a str>,
    pub version: &'a str,
}

impl fmt::Display for MavenCoordinates<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{group_id}:{artifact_id}{packaging}{classifier}:{version}",
            group_id = &self.group_id,
            artifact_id = &self.artifact_id,
            packaging = &self
                .packaging
                .map(|p| format!(":{}", p))
                .unwrap_or_else(|| "".to_string()),
            classifier = &self
                .classifier
                .map(|c| format!(":{}", c))
                .unwrap_or_else(|| "".to_string()),
            version = &self.version,
        )
    }
}

impl<'a> MavenCoordinates<'a> {
    /// Constructs a new, empty `MavenCoordinates`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rvn::MavenCoordinates;
    ///
    /// let coordinates = MavenCoordinates::new("com.fasterxml.jackson.core", "jackson-annotations", None, None, "2.9.9");
    /// ```
    pub fn new(
        group_id: &'a str,
        artifact_id: &'a str,
        packaging: Option<&'a str>,
        classifier: Option<&'a str>,
        version: &'a str,
    ) -> MavenCoordinates<'a> {
        MavenCoordinates {
            group_id,
            artifact_id,
            packaging,
            classifier,
            version,
        }
    }

    /// Parse the Maven coordinates from a string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rvn::MavenCoordinates;
    ///
    /// let coordinates = MavenCoordinates::parse("com.fasterxml.jackson.core:jackson-annotations:2.9.9").unwrap();
    /// ```
    pub fn parse(maven_coordinates: &str) -> Result<MavenCoordinates, &'static str> {
        debug!("Trying to parse Maven coordinates: {}", maven_coordinates);

        // Parse Maven coordinates into named capture groups, with optional packaging OR packaging+classifier
        let regexp = Regex::new(r"^(?P<groupId>[\w.\-]+):(?P<artifactId>[\w.\-]+)(?:(?::(?P<packaging>[\w.\-]+))(?::(?P<classifier>[\w.\-]+)?)?)?:(?P<version>[\w.\-]+)$")
            .expect("Error compiling regex");

        match regexp.captures(maven_coordinates) {
            Some(capture) => Ok(MavenCoordinates::new(
                capture
                    .name("groupId")
                    .map(|m| m.as_str())
                    .expect("Missing groupId"),
                capture
                    .name("artifactId")
                    .map(|m| m.as_str())
                    .expect("Missing artifactId"),
                capture.name("packaging").map(|m| m.as_str()),
                capture.name("classifier").map(|m| m.as_str()),
                capture
                    .name("version")
                    .map(|m| m.as_str())
                    .expect("Missing version"),
            )),
            None => Err("Couldn't parse Maven coordinates"),
        }
    }

    /// Fetch the checksum associated with the Maven coordinates.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rvn::{MavenCoordinates, Algorithm};
    /// use url::Url;
    ///
    /// let repository = Url::parse("https://repo1.maven.org/maven2").unwrap();
    /// let algorithm = Algorithm::Sha1;
    /// let coordinates = MavenCoordinates::parse("com.fasterxml.jackson.core:jackson-annotations:jar:sources:2.9.9").unwrap();
    /// let checksum = coordinates.fetch_checksum(&repository, algorithm).unwrap();
    ///
    /// assert_eq!(checksum, "4ac77aa5799fcf00a9cde00cd7da4d08bdc038ff")
    /// ```
    pub fn fetch_checksum(
        &self,
        repository: &Url,
        algorithm: Algorithm,
    ) -> Result<String, reqwest::Error> {
        let group_id_formatted = str::replace(self.group_id, ".", "/");
        let artifact_uri = format!("{group_id}/{artifact_id}/{version}/{artifact_id}-{version}{classifier}.{packaging}.{algorithm}",
                               group_id = &group_id_formatted,
                               artifact_id = self.artifact_id,
                               version = self.version,
                               classifier = self.classifier.map(|c| format!("-{}", c)).unwrap_or_else(|| "".to_owned()),
                               packaging = self.packaging.unwrap_or("jar"),
                               algorithm = algorithm.to_string());

        let artifact_url = repository
            .clone()
            .append_segment(&artifact_uri)
            .expect("Couldn't append artifact URI to repository URL");

        reqwest::get(artifact_url.as_str())?
            .error_for_status()?
            .text()
    }
}

trait MutSegments {
    fn append_segment(&mut self, uri: &str) -> Result<Url, &'static str>;
}

impl MutSegments for Url {
    fn append_segment(&mut self, uri: &str) -> Result<Url, &'static str> {
        self.path_segments_mut()
            .map_err(|_| "cannot be base")?
            .pop_if_empty()
            .push(&uri);

        Ok(self.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parses_three_component_maven_coordinate() {
        let provided = "com.fasterxml.jackson.core:jackson-annotations:2.9.9";
        let expected = MavenCoordinates::new(
            "com.fasterxml.jackson.core",
            "jackson-annotations",
            None,
            None,
            "2.9.9",
        );

        assert_eq!(MavenCoordinates::parse(provided).unwrap(), expected);
    }

    #[test]
    fn test_parses_four_component_maven_coordinate() {
        let provided = "com.fasterxml.jackson.core:jackson-annotations:pom:2.9.9";
        let expected = MavenCoordinates::new(
            "com.fasterxml.jackson.core",
            "jackson-annotations",
            Some("pom"),
            None,
            "2.9.9",
        );

        assert_eq!(MavenCoordinates::parse(provided).unwrap(), expected);
    }

    #[test]
    fn test_parses_five_component_maven_coordinate() {
        let provided = "com.fasterxml.jackson.core:jackson-annotations:jar:sources:2.9.9";
        let expected = MavenCoordinates::new(
            "com.fasterxml.jackson.core",
            "jackson-annotations",
            Some("jar"),
            Some("sources"),
            "2.9.9",
        );

        assert_eq!(MavenCoordinates::parse(provided).unwrap(), expected);
    }

    #[test]
    fn test_parse_unorthodox_maven_coordinate() {
        let provided = "io.get-coursier:coursier-cli_2.12:jar:standalone:1.1.0-M14-4";
        let expected = MavenCoordinates::new(
            "io.get-coursier",
            "coursier-cli_2.12",
            Some("jar"),
            Some("standalone"),
            "1.1.0-M14-4",
        );

        assert_eq!(MavenCoordinates::parse(provided).unwrap(), expected);
    }
}
