use std::fmt;

use super::{InvalidVersion, Version};

impl fmt::Display for InvalidVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid version: `{}`", self.0)
    }
}

impl std::error::Error for InvalidVersion {}

impl From<&str> for InvalidVersion {
    fn from(v: &str) -> Self {
        InvalidVersion(v.into())
    }
}

impl<'a> TryFrom<&'a str> for Version<'a> {
    type Error = InvalidVersion;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "major" => Ok(Self::Major),
            "minor" => Ok(Self::Minor),
            "patch" => Ok(Self::Patch),
            custom => validate_custom_version(custom),
        }
    }
}

impl<'a> Version<'a> {
    pub fn new(version: &'a str) -> Result<Self, InvalidVersion> {
        version.try_into()
    }
}

fn validate_custom_version(v: &str) -> Result<Version, InvalidVersion> {
    let mut i = 0;
    for part in v.split('.') {
        match i {
            0 | 1 => {
                if !part.chars().all(char::is_numeric) {
                    // major or minor part does not contain number
                    return Err(InvalidVersion(v.into()));
                }
            }
            2 => {
                if !part.chars().all(char::is_numeric) {
                    if let Some(patch) = part.split('-').next() {
                        if !patch.chars().all(char::is_numeric) {
                            return Err(InvalidVersion(v.into()));
                        }
                    } else {
                        // invalid patch format
                        return Err(InvalidVersion(v.into()));
                    }
                }
            }
            _ => return Err(InvalidVersion(v.into())),
        }
        i += 1;
    }
    if i != 3 {
        Err(InvalidVersion(v.into()))
    } else {
        Ok(Version::Custom(v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_try_from(expected: Version, input: &str) {
        let actual = Version::try_from(input).expect("valid version");
        assert_eq!(expected, actual);
    }

    #[test]
    fn new_same_as_try_from() {
        let version = "1.1.1";
        let new = Version::new(version).expect("valid version");
        let from = Version::try_from(version).expect("valid version");
        assert_eq!(new, from);
    }

    #[test]
    fn new_same_as_try_from_error() {
        let version = "bad";
        let new = Version::new(version).expect_err("invalid version");
        let from = Version::try_from(version).expect_err("invalid version");
        assert_eq!(new, from);
    }

    #[test]
    fn try_from_invalid_custom_versions() {
        let invalids = ["1.a.2", "0.0", "0", "v1.2.3", "2.3.4.5", "2.3.4no"];
        for v in invalids {
            let err = Version::try_from(v).expect_err("invalid version");
            let expected = InvalidVersion(v.into());
            assert_eq!(expected, err);
        }
    }

    #[test]
    fn try_from_valid_custom_versions() {
        let valids = ["0.0.0", "0.1.2", "1.2.3", "1.2.3-beta"];
        for v in valids {
            check_try_from(Version::Custom(v), v);
        }
    }

    #[test]
    fn try_from_major() {
        check_try_from(Version::Major, "major");
    }

    #[test]
    fn try_from_minor() {
        check_try_from(Version::Minor, "minor");
    }

    #[test]
    fn try_from_patch() {
        check_try_from(Version::Patch, "patch");
    }
}
