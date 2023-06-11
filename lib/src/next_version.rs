use crate::{InvalidVersion, NextVersion};

impl<'a> TryFrom<&'a str> for NextVersion<'a> {
    type Error = InvalidVersion;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "major" => Ok(Self::Major),
            "minor" => Ok(Self::Minor),
            "patch" => Ok(Self::Patch),
            custom => Ok(Self::Custom(custom.try_into()?)),
        }
    }
}

impl<'a> NextVersion<'a> {
    pub fn new(version: &'a str) -> Result<Self, InvalidVersion> {
        version.try_into()
    }
}

#[cfg(test)]
mod tests {
    use crate::SemanticVersion;

    use super::*;

    fn check_try_from(expected: NextVersion, input: &str) {
        let actual = NextVersion::try_from(input).expect("valid version");
        assert_eq!(expected, actual);
    }

    #[test]
    fn new_same_as_try_from() {
        let version = "1.1.1";
        let new = NextVersion::new(version).expect("valid version");
        let from = NextVersion::try_from(version).expect("valid version");
        assert_eq!(new, from);
    }

    #[test]
    fn new_same_as_try_from_error() {
        let version = "bad";
        let new = NextVersion::new(version).expect_err("invalid version");
        let from = NextVersion::try_from(version).expect_err("invalid version");
        assert_eq!(new, from);
    }

    #[test]
    fn try_from_invalid_custom_versions() {
        let invalids = ["1.a.2", "0.0", "0", "2.3.4.5", "2.3.4no"];
        for v in invalids {
            let actual = match NextVersion::try_from(v) {
                Ok(_) => panic!("input should be invalid: `{}`", v),
                Err(err) => err,
            };
            assert_eq!(InvalidVersion::from(v), actual);
        }
    }

    #[test]
    fn try_from_valid_custom_versions() {
        let valids = ["0.0.0", "0.1.2", "1.50.3", "1.2.3-beta", "v1.2.3"];
        for input in valids {
            let semver = SemanticVersion::try_from(input).expect("valid version");
            check_try_from(NextVersion::Custom(semver), input);
        }
    }

    #[test]
    fn try_from_major() {
        check_try_from(NextVersion::Major, "major");
    }

    #[test]
    fn try_from_minor() {
        check_try_from(NextVersion::Minor, "minor");
    }

    #[test]
    fn try_from_patch() {
        check_try_from(NextVersion::Patch, "patch");
    }
}
