mod changelog;
mod version;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Version<'a> {
    Major,
    Minor,
    Patch,
    Custom(&'a str),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InvalidVersion(String);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Changelog<'c> {
    unreleased_content: Option<&'c str>,
    last_release: Option<Release<'c>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Release<'c> {
    title: &'c str,
    url: Option<&'c str>,
}
