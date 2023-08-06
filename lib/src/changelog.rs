use crate::{Changelog, ReleaseBuilder};

impl<'c> Default for Changelog<'c> {
    fn default() -> Self {
        Self {
            header: None,
            unreleased: ReleaseBuilder::unreleased().build(),
            releases: Vec::new(),
            misc_links: Vec::new(),
        }
    }
}

impl<'c> Changelog<'c> {
    pub fn new() -> Self {
        Self::default()
    }
}
