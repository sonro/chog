use std::{borrow::Cow, cmp::Ordering, fmt};

use crate::{util::optcow_to_owned, InvalidVersion, SemanticVersion};

impl<'v> SemanticVersion<'v> {
    pub fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
            label: None,
        }
    }

    pub fn new_with_label(
        major: u16,
        minor: u16,
        patch: u16,
        label: impl Into<Cow<'v, str>>,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
            label: Some(label.into()),
        }
    }

    pub fn to_owned(&'v self) -> SemanticVersion<'static> {
        let label = optcow_to_owned(self.label.clone());
        SemanticVersion {
            major: self.major,
            minor: self.minor,
            patch: self.patch,
            label,
        }
    }

    pub fn try_owned_from(input: impl AsRef<str>) -> Result<Self, InvalidVersion> {
        let semver = try_from_str(input.as_ref())?;
        Ok(semver.to_owned())
    }
}

impl<'v> TryFrom<&'v str> for SemanticVersion<'v> {
    type Error = InvalidVersion;

    fn try_from(input: &'v str) -> Result<Self, Self::Error> {
        try_from_str(input)
    }
}

// Keep as long function to make use of lazily evaulated error value
fn try_from_str(input: &str) -> Result<SemanticVersion, InvalidVersion> {
    let err = || InvalidVersion::from(input);
    // parse &str into u16 or our lazy error
    let parse_part = |part: &str| part.parse::<u16>().map_err(|_| err());
    // try to parse semver part
    // map a non-existant value to our lazy error
    let part_from_split = |part: Option<&str>| part.ok_or_else(err).map(parse_part)?;

    // remove `v` prefix if it exists
    let input = input.strip_prefix('v').unwrap_or(input);

    let mut parts = input.splitn(3, '.');
    // major.*.*
    let major = part_from_split(parts.next())?;
    // *.minor.*
    let minor = part_from_split(parts.next())?;
    // *.*.patch[-label]
    let (patch, label) = match parts.next() {
        Some(part) => match part.split_once('-') {
            // label (x.x.x-label) exists
            // just parse the patch part as a number
            Some((patch, label)) => (parse_part(patch)?, Some(label.into())),
            // label doesn't exist
            None => (parse_part(part)?, None),
        },
        None => return Err(err()),
    };

    Ok(SemanticVersion {
        major,
        minor,
        patch,
        label,
    })
}

impl<'v> fmt::Display for SemanticVersion<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.label {
            Some(label) => write!(f, "{}.{}.{}-{}", self.major, self.minor, self.patch, label),
            None => write!(f, "{}.{}.{}", self.major, self.minor, self.patch),
        }
    }
}

impl<'v> PartialOrd for SemanticVersion<'v> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<'v> Ord for SemanticVersion<'v> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.major.cmp(&other.major) {
            Ordering::Equal => {}
            ord => return ord,
        }
        match self.minor.cmp(&other.minor) {
            Ordering::Equal => {}
            ord => return ord,
        }
        match self.patch.cmp(&other.patch) {
            Ordering::Equal => {}
            ord => return ord,
        }
        match (&self.label, &other.label) {
            (Some(label), Some(other)) => label.cmp(other),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_str_invalid_versions() {
        let invalids = [
            "1",
            "0",
            "1-label",
            "-1",
            "1.0",
            "0.0",
            "0.0-label",
            "0.0.a",
            "1.a.2",
            "a.2.3",
            "a.b.c",
            "1.2.3.4",
            "1.2.3.label",
            "1.2-label.3",
            "v1.2-test",
        ];
        for v in invalids {
            let actual = match SemanticVersion::try_from(v) {
                Ok(_) => panic!("input should error: `{}`", v),
                Err(err) => err,
            };
            assert_eq!(InvalidVersion::from(v), actual);
        }
    }

    #[test]
    fn try_from_str_valid_versions() {
        for (input, expected) in valid_inputs_and_semvers() {
            let actual = SemanticVersion::try_from(input).expect("valid version");
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn display() {
        for (expected, sv) in valid_inputs_and_semvers() {
            // remove `v` prefix if it exists
            let expected = expected.strip_prefix('v').unwrap_or(expected);
            assert_eq!(expected, sv.to_string());
        }
    }

    #[test]
    fn ordering() {
        let low_to_high = [
            "0.1.0",
            "0.1.1",
            "0.2.0-beta",
            "0.2.0-pr.1",
            "0.2.0-pr.2",
            "0.2.0",
            "1.0.0",
        ];
        for w in low_to_high.windows(2) {
            let low = SemanticVersion::try_from(w[0]).expect("valid version");
            let high = SemanticVersion::try_from(w[1]).expect("valid version");
            assert!(low < high, "`{}` less than `{}`", low, high);
        }
    }

    fn valid_inputs_and_semvers<'v>() -> Vec<(&'v str, SemanticVersion<'v>)> {
        use crate::SemanticVersion as SV;
        vec![
            ("0.0.0", SV::new(0, 0, 0)),
            ("1.1.1", SV::new(1, 1, 1)),
            ("0.1.0", SV::new(0, 1, 0)),
            ("1.0.1", SV::new(1, 0, 1)),
            ("1.2.3", SV::new(1, 2, 3)),
            ("v1.2.3", SV::new(1, 2, 3)),
            ("v0.0.0", SV::new(0, 0, 0)),
            ("10.10.10", SV::new(10, 10, 10)),
            ("200.0.0", SV::new(200, 0, 0)),
            ("0.200.0", SV::new(0, 200, 0)),
            ("0.0.200", SV::new(0, 0, 200)),
            ("0.0.0-beta", SV::new_with_label(0, 0, 0, "beta")),
            (
                "1.2.3-label-is-good.1",
                SV::new_with_label(1, 2, 3, "label-is-good.1"),
            ),
            (
                "v200.200.200-charlie",
                SV::new_with_label(200, 200, 200, "charlie"),
            ),
        ]
    }
}
