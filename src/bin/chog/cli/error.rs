use std::fmt;

use chog::InvalidVersion;

use super::Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownFlag(flag) => write!(f, "unknown flag: `{}`", flag),
            Self::InvalidVersion(version) => version.fmt(f),
            Self::NoVersion => write!(f, "no version provided"),
            Self::NoPath => write!(f, "expected path for given options"),
        }
    }
}

impl From<InvalidVersion> for Error {
    fn from(err: InvalidVersion) -> Self {
        Self::InvalidVersion(err)
    }
}
