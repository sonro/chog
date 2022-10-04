use crate::{release::ReleaseTitle, Changelog, Release, SemanticVersion};

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

const EMPTY_UNRELEASED_CHANGELOG: &str = include_str!("../../test_changelogs/empty_unreleased.md");
const EMPTY_UNRELEASED_CHANGELOG_STRUCT: Changelog = Changelog {
    #[cfg(target_family = "windows")]
    unreleased_content: Some("\r\n"),
    #[cfg(not(target_family = "windows"))]
    unreleased_content: Some("\n"),
    last_release: Some(FULL_LAST_RELEASE),
};

const NO_RELEASE_CHANGELOG: &str = include_str!("../../test_changelogs/no_release.md");
const NO_RELEASE_CHANGELOG_STRUCT: Changelog = Changelog {
    unreleased_content: Some(FULL_UNRELEASED_CONTENT),
    last_release: None,
};

const WEIRD_CHANGELOG: &str = include_str!("../../test_changelogs/full_weird_link.md");
const FULL_CHANGELOG: &str = include_str!("../../test_changelogs/full.md");
const FULL_CHANGELOG_STRUCT: Changelog = Changelog {
    unreleased_content: Some(FULL_UNRELEASED_CONTENT),
    last_release: Some(FULL_LAST_RELEASE),
};

const FULL_UNRELEASED_CONTENT: &str = include_str!("../../test_changelogs/just_unreleased.md");
const FULL_LAST_RELEASE: Release = Release {
    title: ReleaseTitle::SemVer(SemanticVersion {
        major: 1,
        minor: 0,
        patch: 0,
        label: None,
    }),
    url: Some("https://github.com/user/repo/releases/tag/v1.0.0"),
};
