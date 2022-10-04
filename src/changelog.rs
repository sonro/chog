use crate::{Changelog, Release};

const UNRELEASED_TITLE: &str = "## [Unreleased]";
const RELEASE_PREFIX: &str = "## [";

impl<'c> Changelog<'c> {
    pub fn new(input: &'c str) -> Self {
        Self {
            unreleased_content: parse_unreleased(input),
            last_release: parse_last_release(input),
        }
    }
}

impl<'c> From<&'c str> for Changelog<'c> {
    fn from(input: &'c str) -> Self {
        Changelog::new(input)
    }
}

fn parse_unreleased(input: &str) -> Option<&str> {
    match input.find(UNRELEASED_TITLE) {
        Some(idx) => {
            let start = match input[idx..].find('\n') {
                Some(start) => start + idx + 1,
                // no content if no newline
                None => return None,
            };
            match input[start..].find(RELEASE_PREFIX) {
                // from after unreleased title until last release title
                Some(end) => Some(&input[start..end + start]),
                // no other releases
                None => match input[start..].find("[Unreleased]:") {
                    // until links at bottom
                    Some(end) => Some(&input[start..end + start]),
                    // until eof
                    None => Some(&input[start..]),
                },
            }
        }
        // no unreleased section
        None => None,
    }
}

fn parse_last_release(input: &str) -> Option<Release> {
    let title = match input.find(UNRELEASED_TITLE) {
        Some(idx) => match_release_title(&input[idx + UNRELEASED_TITLE.len()..]),
        None => match_release_title(input),
    }?;

    let clean_title = &title[1..title.len() - 1];

    Some(Release {
        title: clean_title.into(),
        url: match_release_url(input, title),
    })
}

fn match_release_title(input: &str) -> Option<&str> {
    match input.find(RELEASE_PREFIX) {
        Some(idx) => {
            let start = idx + RELEASE_PREFIX.len();
            match input[start..].find(']') {
                Some(end) => Some(&input[(start - 1)..(end + start + 1)]),
                None => None,
            }
        }
        None => None,
    }
}

fn match_release_url<'c>(input: &'c str, title: &'c str) -> Option<&'c str> {
    for (i, _) in input.rmatch_indices(title) {
        let next = i + title.len() + 1;
        if &input[next - 1..next] == ":" {
            // we have a link
            match input[next..].find('\n') {
                Some(end) => return Some(input[next..end + next].trim()),
                // EOF
                None => return Some(input[next..].trim()),
            }
        } else if &input[next - 1..next] == "(" {
            // we have a link
            if let Some(end) = input[next..].find(')') {
                return Some(input[next..end + next].trim());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{release::ReleaseTitle, SemanticVersion};

    use super::*;

    #[test]
    fn new_full_input() {
        let input = FULL_CHANGELOG;
        let chog = Changelog::new(input);
        assert_eq!(FULL_CHANGELOG_STRUCT, chog);
    }

    #[test]
    fn new_full_weird_link_input() {
        let input = WEIRD_CHANGELOG;
        let chog = Changelog::new(input);
        assert_eq!(FULL_CHANGELOG_STRUCT, chog);
    }

    #[test]
    fn new_no_release_input() {
        // add extra newline to input
        // easier than trimming one off expected
        #[cfg(target_family = "windows")]
        let input = format!("{}\r\n", NO_RELEASE_CHANGELOG);
        #[cfg(not(target_family = "windows"))]
        let input = format!("{}\n", NO_RELEASE_CHANGELOG);

        let chog = Changelog::new(&input);
        assert_eq!(NO_RELEASE_CHANGELOG_STRUCT, chog);
    }

    #[test]
    fn new_empty_unreleased_input() {
        let input = EMPTY_UNRELEASED_CHANGELOG;
        let chog = Changelog::new(input);
        assert_eq!(EMPTY_UNRELEASED_CHANGELOG_STRUCT, chog);
    }

    const EMPTY_UNRELEASED_CHANGELOG: &str = include_str!("../test_changelogs/empty_unreleased.md");
    const EMPTY_UNRELEASED_CHANGELOG_STRUCT: Changelog = Changelog {
        #[cfg(target_family = "windows")]
        unreleased_content: Some("\r\n"),
        #[cfg(not(target_family = "windows"))]
        unreleased_content: Some("\n"),
        last_release: Some(FULL_LAST_RELEASE),
    };

    const NO_RELEASE_CHANGELOG: &str = include_str!("../test_changelogs/no_release.md");
    const NO_RELEASE_CHANGELOG_STRUCT: Changelog = Changelog {
        unreleased_content: Some(FULL_UNRELEASED_CONTENT),
        last_release: None,
    };

    const WEIRD_CHANGELOG: &str = include_str!("../test_changelogs/full_weird_link.md");
    const FULL_CHANGELOG: &str = include_str!("../test_changelogs/full.md");
    const FULL_CHANGELOG_STRUCT: Changelog = Changelog {
        unreleased_content: Some(FULL_UNRELEASED_CONTENT),
        last_release: Some(FULL_LAST_RELEASE),
    };

    const FULL_UNRELEASED_CONTENT: &str = include_str!("../test_changelogs/just_unreleased.md");
    const FULL_LAST_RELEASE: Release = Release {
        title: ReleaseTitle::SemVer(SemanticVersion {
            major: 1,
            minor: 0,
            patch: 0,
            label: None,
        }),
        url: Some("https://github.com/user/repo/releases/tag/v1.0.0"),
    };
}
