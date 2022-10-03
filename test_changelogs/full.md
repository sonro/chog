# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `Config` pretty printing.

### Documentation

- Fix typos in readme.
- Add example of `Config` pretty printing.

## [1.0.0] - 2022-10-03

- Stabilized API

### Documentation

- More examples.

## [1.0.0-beta] - 2022-09-28

### Added

- Builder interface to make creating Config easier.

### Changed

- Fix extra newlines appearing in debug output.
- [**BREAKING CHANGE**] Remove deprecated `Config::new` method.

## [0.2.0] - 2022-09-20

### Changed

- Deprecate `Config::new`.
- Fix over-zealous errors.
- [**BREAKING CHANGE**] `Config` can no longer be converted from `App`.

## [0.1.1] - 2022-09-10

### Added

- Allow `App` to be converted to `Config`.
- Robust error handling.

## [0.1.0] - 2022-09-01

### Added

- `App` and `Config` APIs

[Unreleased]: https://github.com/user/repo/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/user/repo/releases/tag/v1.0.0
[1.0.0-beta]: https://github.com/user/repo/releases/tag/v1.0.0-beta
[0.2.0]: https://github.com/user/repo/releases/tag/v0.2.0
[0.1.1]: https://github.com/user/repo/releases/tag/v0.1.1
[0.1.0]: https://github.com/user/repo/releases/tag/v0.1.0
