mod app;

use std::fmt;

pub use app::App;
use chog::InvalidVersion;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Error {
    UnknownFlag(String),
    InvalidVersion(InvalidVersion),
    NoVersion,
    NoPath,
}

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
