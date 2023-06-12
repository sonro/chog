use crate::Changelog;

mod parse;

impl<'c> Changelog<'c> {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LinkFooter<'c> {
    unreleased_link: Option<&'c str>,
    version_links: Vec<&'c str>,
    mis_links: Vec<&'c str>,
}
