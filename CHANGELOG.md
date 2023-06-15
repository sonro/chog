# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### [**BREAKING**] Project changes

- Cli and Lib now separate packages.
- Lib is now based on parsing and manipulating a CHANGELOG.md file, not just
  for updating the "Unreleased" section.
- Bin is now a development tool not just for updating the "Unreleased" section.

### Changed

- [**BREAKING**] All types that had borrowed strings now can be either owned or
  borrowed.
- [**BREAKING**] Full Changelog representation with all releases and their
  dates.

### Added

#### Cli

- Command line argument handling.
- Usage instructions for `--help` and invalid arguments.

#### Lib

- `NextVersion` type.
- Changelog parsing with `Changelog` and `Release` types.
- `SemanticVersion` type.

## [0.1.0] - 2022-10-02

- Initialized project.

[Unreleased]: https://github.com/sonro/narrate/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/sonro/narrate/releases/tag/v0.1.0
