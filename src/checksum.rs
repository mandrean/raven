extern crate strum;

use std::fmt;
use strum_macros::{Display, EnumString, EnumVariantNames, IntoStaticStr};

#[derive(Debug, Eq, PartialEq)]
pub struct Checksum {
    pub algorithm: Algorithm,
    pub value: String,
}

impl Checksum {
    /// Constructs a new, empty `Checksum`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rvn::checksum::{Checksum, Algorithm};
    ///
    /// let checksum = Checksum::new(Algorithm::Sha1, "4ac77aa5799fcf00a9cde00cd7da4d08bdc038ff");
    ///
    /// assert_eq!(checksum.algorithm, Algorithm::Sha1);
    /// assert_eq!(checksum.value, "4ac77aa5799fcf00a9cde00cd7da4d08bdc038ff");
    /// ```
    pub fn new(algorithm: Algorithm, value: &str) -> Checksum {
        Checksum {
            algorithm,
            value: value.to_owned(),
        }
    }
}

impl fmt::Display for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{algorithm}:{value}",
            algorithm = &self.algorithm.to_string(),
            value = &self.value,
        )
    }
}

#[derive(EnumString, EnumVariantNames, Debug, Display, Eq, PartialEq, IntoStaticStr)]
#[strum(serialize_all = "kebab_case")]
pub enum Algorithm {
    Md5,
    Sha1,
    Sha256,
    Sha384,
    Sha512,
}
