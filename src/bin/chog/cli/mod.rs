use std::path::Path;

use chog::{InvalidVersion, NextVersion};

mod app;
mod error;
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct App<'a> {
    pub help: bool,
    pub info: bool,
    pub quiet: bool,
    pub force: bool,
    pub dry_run: bool,
    pub version: NextVersion<'a>,
    pub in_file: Option<&'a Path>,
    pub out_file: Option<&'a Path>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Error {
    UnknownFlag(String),
    InvalidVersion(InvalidVersion),
    NoVersion,
    NoPath,
}
