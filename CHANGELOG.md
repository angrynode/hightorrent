# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version UNRELEASED (XXXX-YY-ZZ)

### Added

- `SingleTarget::matches_hash` compares a single target request with a given `InfoHash` ([#5](https://github.com/angrynode/hightorrent/pull/5))

### Changed

- `MagnetLink` can now be built with no `dn` (magnet name), unless the `magnet_force_name` crate feature is enabled ([#6](https://github.com/angrynode/hightorrent/pull/6))

### Fixed

- `TorrentList::get` now properly matches truncated V2 hash requests ([#5](https://github.com/angrynode/hightorrent/pull/5))
- `SingleTarget::new` now normalizes the requested hash just like `InfoHash` does, to avoid issues when
  dealing with different casing ([#4](https://github.com/angrynode/hightorrent/pull/4))

## Version 0.1.0 (2022-12-22)

### Added

- Initial release
