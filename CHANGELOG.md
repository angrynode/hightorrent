# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version UNRELEASED (XXXX-YY-ZZ)

### Added

- `SingleTarget::matches_hash` compares a single target request with a given `InfoHash`

### Fixed

- `TorrentList::get` now properly matches truncated V2 hash requests
- `SingleTarget::new` now normalizes the requested hash just like `InfoHash` does, to avoid issues when
  dealing with different casing

## Version 0.1.0 (2022-12-22)

### Added

- Initial release
