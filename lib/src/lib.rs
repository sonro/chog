use std::borrow::Cow;

use changelog::LinkFooter;
use release::ReleaseTitle;

mod changelog;
mod error;
mod next_version;
mod release;
mod semver;
mod util;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NextVersion<'a> {
    Major,
    Minor,
    Patch,
    Custom(SemanticVersion<'a>),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SemanticVersion<'v> {
    major: u16,
    minor: u16,
    patch: u16,
    label: Option<Cow<'v, str>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InvalidVersion(String);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Changelog<'c> {
    header: Option<Cow<'c, str>>,
    unreleased: Option<Release<'c>>,
    releases: Vec<Release<'c>>,
    link_footer: Option<LinkFooter<'c>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Release<'c> {
    title: ReleaseTitle<'c>,
    url: Option<Cow<'c, str>>,
    date: Option<Cow<'c, str>>,
    content: Option<Cow<'c, str>>,
}
