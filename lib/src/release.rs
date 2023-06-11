use std::{cmp::Ordering, fmt};

use crate::{Release, SemanticVersion};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ReleaseTitle<'c> {
    Title(&'c str),
    SemVer(SemanticVersion<'c>),
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
            Err(_) => Self::Title(input),
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
        let low_to_high = ["0.1.1", "0.1.2", "0.0.0.1", "bad-title", "later-bad-title"];

        for w in low_to_high.windows(2) {
            let low = ReleaseTitle::from(w[0]);
            let high = ReleaseTitle::from(w[1]);
            assert!(low < high, "`{}` less than `{}`", low, high);
        }
    }
}
