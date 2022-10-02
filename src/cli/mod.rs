mod app;

use std::fmt;

pub use app::App;

#[derive(Debug)]
pub enum Error {
    UnknownFlag(String),
    InvalidVersion(String),
    NoArgs,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownFlag(flag) => write!(f, "unknown flag: `{}`", flag),
            Self::InvalidVersion(version) => write!(f, "invalid custom version: `{}`", version),
            Self::NoArgs => write!(f, "no version"),
        }
    }
}
