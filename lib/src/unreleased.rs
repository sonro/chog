use std::borrow::Cow;

use crate::{release::ReleaseTitle, Release, Unreleased};

impl<'c> Unreleased {
    pub fn with<U, C>(url: U, content: C) -> Release<'c>
    where
        U: Into<Cow<'c, str>>,
        C: Into<Cow<'c, str>>,
    {
        Self::build(Some(url.into()), Some(content.into()))
    }

    pub fn with_content<C: Into<Cow<'c, str>>>(content: C) -> Release<'c> {
        Self::build(None, Some(content.into()))
    }

    pub fn with_url<U: Into<Cow<'c, str>>>(url: U) -> Release<'c> {
        Self::build(Some(url.into()), None)
    }

    pub fn empty() -> Release<'c> {
        Self::build(None, None)
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
