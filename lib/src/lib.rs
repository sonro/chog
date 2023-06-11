use release::ReleaseTitle;

mod changelog;
mod error;
mod next_version;
mod release;
mod semver;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NextVersion<'a> {
    Major,
    Minor,
    Patch,
    Custom(SemanticVersion<'a>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SemanticVersion<'v> {
    major: u16,
    minor: u16,
    patch: u16,
    label: Option<&'v str>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InvalidVersion(String);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Changelog<'c> {
    unreleased_content: Option<&'c str>,
    last_release: Option<Release<'c>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Release<'c> {
    title: ReleaseTitle<'c>,
    url: Option<&'c str>,
}
