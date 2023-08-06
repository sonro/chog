use std::borrow::Cow;

use crate::{
    release::ReleaseTitle,
    util::{trim_to_optcow_borrow, trim_to_optcow_owned},
    Changelog, ChangelogBuilder, Release,
};

impl<'c> Default for ChangelogBuilder<'c> {
    fn default() -> Self {
        Self {
            changelog: Changelog::new(),
        }
    }
}

impl<'c> ChangelogBuilder<'c> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Changelog<'c> {
        self.changelog
    }

    pub fn header(mut self, header: &'c str) -> Self {
        self.changelog.header = trim_to_optcow_borrow(header);
        self
    }

    pub fn header_own(mut self, header: String) -> Self {
        self.changelog.header = trim_to_optcow_owned(header);
        self
    }

    pub fn unreleased(mut self, mut unreleased: Release<'c>) -> Self {
        unreleased.date = None;
        unreleased.title = ReleaseTitle::unreleased();
        self.changelog.unreleased = unreleased;
        self
    }

    pub fn add_release(mut self, release: Release<'c>) -> Self {
        self.changelog.releases.push(release);
        self
    }

    pub fn releases(mut self, releases: Vec<Release<'c>>) -> Self {
        self.changelog.releases = releases;
        self
    }

    pub fn add_misc_link(mut self, link: &'c str) -> Self {
        self.changelog.misc_links.push(Cow::Borrowed(link));
        self
    }

    pub fn add_misc_link_own(mut self, link: String) -> Self {
        self.changelog.misc_links.push(Cow::Owned(link));
        self
    }

    pub fn misc_links(mut self, links: &[&'c str]) -> Self {
        self.changelog.misc_links = links.iter().map(|&l| Cow::Borrowed(l)).collect();
        self
    }

    pub fn misc_links_own(mut self, links: Vec<String>) -> Self {
        self.changelog.misc_links = links.into_iter().map(Cow::Owned).collect();
        self
    }
}
