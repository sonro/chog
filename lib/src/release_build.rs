use std::borrow::Cow;

use crate::{
    util::{trim_to_optcow_borrow, trim_to_optcow_owned},
    Release, ReleaseBuilder, ReleaseTitle, SemanticVersion,
};

impl<'c> ReleaseBuilder<'c> {
    pub fn unreleased() -> Self {
        Self::new_with_title(ReleaseTitle::unreleased())
    }

    pub fn semver(semver: SemanticVersion<'c>) -> Self {
        Self::new_with_title(ReleaseTitle::SemVer(semver))
    }

    pub fn title(title: &'c str) -> Self {
        Self::new_with_title(ReleaseTitle::Title(Cow::Borrowed(title)))
    }

    pub fn title_own(title: String) -> Self {
        Self::new_with_title(ReleaseTitle::Title(Cow::Owned(title)))
    }

    fn new_with_title(title: ReleaseTitle<'c>) -> Self {
        Self {
            release: Release {
                title,
                url: None,
                date: None,
                content: None,
            },
        }
    }

    pub fn build(self) -> Release<'c> {
        self.release
    }

    pub fn url(mut self, url: &'c str) -> Self {
        self.release.url = trim_to_optcow_borrow(url);
        self
    }

    pub fn url_own(mut self, url: String) -> Self {
        self.release.url = trim_to_optcow_owned(url);
        self
    }

    pub fn content(mut self, content: &'c str) -> Self {
        self.release.content = trim_to_optcow_borrow(content);
        self
    }

    pub fn content_own(mut self, content: String) -> Self {
        self.release.content = trim_to_optcow_owned(content);
        self
    }

    pub fn date(mut self, date: &'c str) -> Self {
        self.release.content = trim_to_optcow_borrow(date);
        self
    }

    pub fn date_own(mut self, date: String) -> Self {
        self.release.content = trim_to_optcow_owned(date);
        self
    }
}
