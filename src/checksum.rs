extern crate strum;

use strum_macros::{Display, EnumString, EnumVariantNames, IntoStaticStr};

#[derive(EnumString, EnumVariantNames, Debug, Display, PartialEq, IntoStaticStr)]
#[strum(serialize_all = "kebab_case")]
pub enum Algorithm {
    Md5,
    Sha1,
    Sha256,
    Sha384,
    Sha512,
}
