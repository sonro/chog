# chog

`chog` is a simple program to bump the version in a CHANGELOG file. Following
the format layed out in [Keep a Changelog], everything in `Unreleased` will be
moved into a new section; named after the new release. The GitHub comparison
links are updated to the latest tags. Ideally used for manually updated
changelogs.

[![license](https://img.shields.io/crates/l/chog.svg)](#license)

## Recommended environment

`chog` is highly opinionated. In order for it to be effective it is recommended
to:

- Your formatting follows [Keep a Changelog].
- Use git as your version control system.
- Host your repository on GitHub.
- Tag your versions using the v0.0.0 style.
- Use [Semantic Versioning](https://semver.org/).
- Your file name is CHANGELOG.md.

## License

chog is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
