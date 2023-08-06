use std::borrow::Cow;

use crate::{
    release::ReleaseTitle,
    util::{trim_to_optcow_borrow, trim_to_optcow_owned},
    Release, Unreleased,
};

impl<'c> Unreleased {
    pub fn with(url: &'c str, content: &'c str) -> Release<'c> {
        let url = trim_to_optcow_borrow(url);
        let content = trim_to_optcow_borrow(content);
        Self::build(url, content)
    }

    pub fn with_content(content: &'c str) -> Release<'c> {
        let content = trim_to_optcow_borrow(content);
        Self::build(None, content)
    }

    pub fn with_url(url: &'c str) -> Release<'c> {
        let url = trim_to_optcow_borrow(url);
        Self::build(url, None)
    }

    pub fn with_own(url: String, content: String) -> Release<'static> {
        let url = trim_to_optcow_owned(url);
        let content = trim_to_optcow_owned(content);
        Self::build(url, content)
    }

    pub fn with_url_own(url: String) -> Release<'static> {
        let url = trim_to_optcow_owned(url);
        Self::build(url, None)
    }

    pub fn with_content_own(content: String) -> Release<'static> {
        let content = trim_to_optcow_owned(content);
        Self::build(None, content)
    }

    fn build(url: Option<Cow<'c, str>>, content: Option<Cow<'c, str>>) -> Release<'c> {
        Release {
            title: ReleaseTitle::unreleased(),
            date: None,
            url,
            content,
        }
    }
}
