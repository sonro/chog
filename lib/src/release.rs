use std::{borrow::Cow, cmp::Ordering, fmt};

use crate::{
    util::{optcow, optcow_to_owned},
    Release, SemanticVersion,
};

impl<'c> Release<'c> {
    pub fn title_string(&self) -> String {
        self.title.to_string()
    }

    pub fn content(&self) -> Option<&str> {
        self.content.as_deref()
    }

    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    pub fn date(&self) -> Option<&str> {
        self.date.as_deref()
    }

    pub fn set_content<T: Into<Cow<'c, str>>>(&mut self, content: T) {
        self.content = optcow(content);
    }

    pub fn mut_content(&mut self) -> &mut String {
        match self.content {
            Some(ref mut cow) => cow.to_mut(),
            None => {
                self.content = Some(Cow::Owned(String::new()));
                self.content.as_mut().unwrap().to_mut()
            }
        }
    }

    pub fn set_url<T: Into<Cow<'c, str>>>(&mut self, url: T) {
        self.url = optcow(url);
    }

    pub fn to_owned(&self) -> Release<'static> {
        Release {
            title: self.title.to_owned(),
            url: optcow_to_owned(self.url.clone()),
            date: optcow_to_owned(self.date.clone()),
            content: optcow_to_owned(self.content.clone()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ReleaseTitle<'c> {
    Title(Cow<'c, str>),
    SemVer(SemanticVersion<'c>),
}

impl<'c> ReleaseTitle<'c> {
    pub fn unreleased() -> ReleaseTitle<'static> {
        ReleaseTitle::Title(Cow::Borrowed("Unreleased"))
    }

    pub fn to_owned(&self) -> ReleaseTitle<'static> {
        match self {
            ReleaseTitle::SemVer(semver) => ReleaseTitle::SemVer(semver.to_owned()),
            ReleaseTitle::Title(Cow::Owned(title)) => ReleaseTitle::Title(title.to_owned().into()),
            ReleaseTitle::Title(Cow::Borrowed(title)) => {
                ReleaseTitle::Title(String::from(*title).into())
            }
        }
    }
}

impl<'c> PartialOrd for Release<'c> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<'c> Ord for Release<'c> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.title.cmp(&other.title)
    }
}

impl<'c> PartialOrd for ReleaseTitle<'c> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<'c> Ord for ReleaseTitle<'c> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Title(_), Self::SemVer(_)) => Ordering::Greater,
            (Self::SemVer(_), Self::Title(_)) => Ordering::Less,
            (Self::Title(l), Self::Title(r)) => l.cmp(r),
            (Self::SemVer(l), Self::SemVer(r)) => l.cmp(r),
        }
    }
}

impl<'c> From<&'c str> for ReleaseTitle<'c> {
    fn from(input: &'c str) -> Self {
        match SemanticVersion::try_from(input) {
            Ok(semver) => Self::SemVer(semver),
            Err(_) => Self::Title(Cow::Borrowed(input)),
        }
    }
}

impl From<String> for ReleaseTitle<'static> {
    fn from(input: String) -> Self {
        match SemanticVersion::try_owned_from(&input) {
            Ok(semver) => Self::SemVer(semver),
            _ => Self::Title(Cow::Owned(input)),
        }
    }
}

impl<'c> fmt::Display for ReleaseTitle<'c> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Title(s) => s.fmt(f),
            Self::SemVer(s) => s.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering() {
        let low_to_high = [
            "0.1.1",
            "0.1.2",
            "1.0.0-alpha",
            "1.0.0-beta",
            "1.0.0",
            "1.0.1",
            "1.1.0",
            "1.1.1",
            "0.0.0.1",
            "bad-title",
            "later-bad-title",
        ];

        for w in low_to_high.windows(2) {
            let low = ReleaseTitle::from(w[0]);
            let high = ReleaseTitle::from(w[1]);
            assert!(low < high, "`{}` less than `{}`", low, high);
        }
    }
}
