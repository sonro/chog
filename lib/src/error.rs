use std::fmt;

use crate::InvalidVersion;

impl fmt::Display for InvalidVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid version: `{}`", self.0)
    }
}

impl std::error::Error for InvalidVersion {}

impl From<&str> for InvalidVersion {
    fn from(v: &str) -> Self {
        InvalidVersion(v.into())
    }
}
