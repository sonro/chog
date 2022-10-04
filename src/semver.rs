use std::fmt;

use crate::{InvalidVersion, SemanticVersion};

impl<'v> SemanticVersion<'v> {
    pub fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
            label: None,
        }
    }

    pub fn new_with_label(major: u16, minor: u16, patch: u16, label: &'v str) -> Self {
        let label = Some(label);
        Self {
            major,
            minor,
            patch,
            label,
        }
    }
}

impl<'v> TryFrom<&'v str> for SemanticVersion<'v> {
    type Error = InvalidVersion;

    // Keep as long function to make use of lazily evaulated error value
    fn try_from(input: &'v str) -> Result<Self, Self::Error> {
        let err = || InvalidVersion::from(input);
        // parse &str into u16 or our lazy error
        let parse_part = |part: &str| part.parse::<u16>().map_err(|_| err());
        // try to parse semver part
        // map a non-existant value to our lazy error
        let part_from_split = |part: Option<&str>| part.ok_or_else(err).map(parse_part)?;

        // remove `v` prefix if it exists
        let input = input.strip_prefix('v').unwrap_or(input);

        let mut parts = input.split('.');
        // major.*.*
        let major = part_from_split(parts.next())?;
        // *.minor.*
        let minor = part_from_split(parts.next())?;
        // *.*.patch[-label]
        let (patch, label) = match parts.next() {
            Some(part) => match part.split_once('-') {
                // label (x.x.x-label) exists
                // just parse the patch part as a number
                Some((patch, label)) => (parse_part(patch)?, Some(label)),
                // label doesn't exist
                None => (parse_part(part)?, None),
            },
            None => return Err(err()),
        };

        match parts.next() {
            // too many parts for semver e.g. 1.2.3.4
            Some(_) => Err(err()),
            None => Ok(Self {
                major,
                minor,
                patch,
                label,
            }),
        }
    }
}

impl<'v> fmt::Display for SemanticVersion<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.label {
            Some(label) => write!(f, "{}.{}.{}-{}", self.major, self.minor, self.patch, label),
            None => write!(f, "{}.{}.{}", self.major, self.minor, self.patch),
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
                "1.2.3-label-is-good",
                SV::new_with_label(1, 2, 3, "label-is-good"),
            ),
            (
                "v200.200.200-charlie",
                SV::new_with_label(200, 200, 200, "charlie"),
            ),
        ]
    }
}
