# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## UNRELEASED (YYYY-MM-DD)

## Version 0.3.0 (2025-08-27)

This release focuses on supporting listing files contained in torrents. This is not implemented for magnet files, but is implemented for `TorrentFile` and will be implemented in [hightorrent_api](https://github.com/angrynode/hightorrent_api) for the QBittorrent backend.

### Added

- `DecodedInfo.piece_length` contains the torrent piece length in bytes, with a maximum supported size of `536854528` like in libtorrent
- `TorrentContent` represents a file in a torrent ; `ToTorrentContent` is a trait enabling specialized representations to be turned into a backend-agnostic `TorrentContent` ; padding files are ignored when producing a list of content files
- `DecodedTorrent::files()` produces the file list in the torrent (only v1 torrents supported for now)
- `IntoIterator` is now implemented for `&TorrentList`

### Changed

- Not having a `piece length` info field in a torrent produces an error ; so does having a size exceeding `536854528` bytes
- Having `/` or `..` in a content file part produces a `TorrentFileError::InvalidContentPath`

### Meta

- Added more test cases from arvidn/libtorrent to make sure we don't allow parsing invalid torrents

## Version 0.2.0 (2024-09-02)

### Added

- `SingleTarget::matches_hash` compares a single target request with a given `InfoHash` ([#5](https://github.com/angrynode/hightorrent/pull/5))

### Changed

- `IntoTorrent` trait has been renamed `ToTorrent`, and the associated method is now `to_torrent` ([#8](https://github.com/angrynode/hightorrent/pull/8))
- `MagnetLink` can now be built with no `dn` (magnet name), unless the `magnet_force_name` crate feature is enabled ([#7](https://github.com/angrynode/hightorrent/pull/7))

### Fixed

- `TorrentList::get` now properly matches truncated V2 hash requests ([#5](https://github.com/angrynode/hightorrent/pull/5))
- `SingleTarget::new` now normalizes the requested hash just like `InfoHash` does, to avoid issues when
  dealing with different casing ([#4](https://github.com/angrynode/hightorrent/pull/4))

## Version 0.1.0 (2022-12-22)

### Added

- Initial release
