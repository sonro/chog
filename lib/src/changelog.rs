use crate::{Changelog, Unreleased};

impl<'c> Default for Changelog<'c> {
    fn default() -> Self {
        Self {
            header: None,
            unreleased: Unreleased::empty(),
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
