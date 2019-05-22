extern crate regex;
extern crate reqwest;
extern crate url;

use regex::Regex;
use reqwest::Error;
use url::Url;

/// A parsed Maven coordinates record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MavenCoordinates<'a> {
    pub group_id: &'a str,
    pub artifact_id: &'a str,
    pub packaging: Option<&'a str>,
    pub classifier: Option<&'a str>,
    pub version: &'a str,
}

impl<'a> MavenCoordinates<'a> {
    /// Parse the Maven coordinates from a string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rvn::MavenCoordinates;
    ///
    /// let coordinates = MavenCoordinates::parse("com.fasterxml.jackson.core:jackson-annotations:2.9.9").unwrap();
    /// ```
    pub fn parse(maven_coordinates: &str) -> Result<MavenCoordinates, Error> {
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

    /// Fetch the checksum associated with the Maven coordinates.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rvn::MavenCoordinates;
    ///
    /// let repository = "https://repo1.maven.org/maven2";
    /// let algorithm = "sha1";
    /// let coordinates = MavenCoordinates::parse("com.fasterxml.jackson.core:jackson-annotations:jar:sources:2.9.9").unwrap();
    /// let checksum = coordinates.fetch_checksum(repository, "sha1").unwrap();
    ///
    /// assert_eq!(checksum, "4ac77aa5799fcf00a9cde00cd7da4d08bdc038ff")
    /// ```
    pub fn fetch_checksum(&self, repository: &str, algorithm: &str) -> Result<String, Error> {
        let group_id_formatted = str::replace(self.group_id, ".", "/");
        let join_uri = format!("{group_id}/{artifact_id}/{version}/{artifact_id}-{version}{classifier}.{packaging}.{algorithm}",
                               group_id = &group_id_formatted,
                               artifact_id = self.artifact_id,
                               version = self.version,
                               classifier = self.classifier.map(|c| format!("-{}", c)).unwrap_or("".to_string()),
                               packaging = self.packaging.unwrap_or("jar"),
                               algorithm = algorithm);

        let mut url = Url::parse(repository).unwrap();
        url.path_segments_mut().map_err(|_| "cannot be base").unwrap()
            .pop_if_empty().push(&join_uri);

        let mut response = reqwest::get(url).unwrap();

        let checksum = response.text()?;
        Ok(checksum)
    }
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
        assert_eq!(MavenCoordinates::parse(provided).unwrap(), expected);
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
        assert_eq!(MavenCoordinates::parse(provided).unwrap(), expected);
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
        assert_eq!(MavenCoordinates::parse(provided).unwrap(), expected);
    }
}
