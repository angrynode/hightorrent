# HighTorrent

<!-- cargo-rdme start -->

HighTorrent is a library which contains high-level data structures and functions
to interact with Bittorrent v1 and v2 torrents. HighTorrent does not aim to be featureful,
but rather to be super easy to use and interoperate with more advanced torrent software,
and completely impossible to misuse.

**Note that HighTorrent is not a networked library. It will not provide any utilities for
querying the DHT and/or downloading torrents. HighTorrent is much lower in the stack.**

HighTorrent provides utilities to extract name and hash from torrents/magnets, using the
[`MagnetLink`](https://docs.rs/hightorrent/latest/hightorrent/magnet/struct.MagnetLink.html) and [`TorrentFile`](https://docs.rs/hightorrent/latest/hightorrent/torrent_file/struct.TorrentFile.html) structures, but could provide more advanced utilities in the future (PRs welcome). Additionally, it provides the [`Torrent`](https://docs.rs/hightorrent/latest/hightorrent/torrent/struct.Torrent.html) struct and the
[`ToTorrent`](https://docs.rs/hightorrent/latest/hightorrent/torrent/trait.ToTorrent.html) trait representing fully-loaded torrents ; those helpers are intended to be used by more diverse torrenting libraries to provide interoperability out-of-the-box.

Finally, the [`SingleTarget`](https://docs.rs/hightorrent/latest/hightorrent/target/struct.SingleTarget.html) and
[`MultiTarget`](https://docs.rs/hightorrent/latest/hightorrent/target/enum.MultiTarget.html) structures represent one or more torrents you wish to
interact with. The contained stringy value is ambiguous, and can represent either a precise
[`InfoHash`](https://docs.rs/hightorrent/latest/hightorrent/hash/enum.InfoHash.html) or a libtorrent-compatible [`TorrentID`](https://docs.rs/hightorrent/latest/hightorrent/id/struct.TorrentID.html) (truncated hash).

<!-- cargo-rdme end -->

# Related projects

- [intermodal](https://github.com/casey/intermodal): CLI program for managing torrents, but does not expose a library crate

Do you know other related Rust projects? Please let us know.

This library was developed as part of the TorrentManager project, because there was no high-level library to read magnet links and torrent files that would support Bittorrent v2 torrents. New features will be added as they are required by the larger project, but suggestions are welcome if this library can benefit other people and projects as well.

# Contribution

Contributions are welcome. Here are the steps to make sure your contribution gets merged:

- open a PR on [Github angrynode/hightorrent](https://github.com/angrynode/hightorrent/).
- make sure tests pass with `just check`; it's running more steps than just `cargo test`
- if you changed the Rust crate docs, don't forget to run `cargo rdme` to update the README

If you don't have those dependencies (`just`, `cargo-rdme`), you can setup a temporary development environment with [Nix](https://nixos.org/) by running `nix develop`.

# Running tests

From the repository root, run `cargo test`. To run advanced tests using rust nightly as used in CI, run `scripts/pre-commit.sh`. To run the test verifying that error cases from libtorrent are properly handled (which is normally ignored), run `cargo test -- --ignored`.

# Possible improvements for v1

- [x] hand-implement errors to remove snafu dependency
- [ ] provide one-off methods to read name/hash from torrent/magnet
- [ ] store hashes as integers (not strings) for optimization
- [ ] make Tracker differentiate HTTP/HTTPS trackers
- [ ] implement MultiTarget filtering, including boolean logic (AND/OR/XOR)
- [ ] provide more information for TorrentFile (eg. files list)
- [ ] consider replacing Torrent with a trait
- [ ] implement more libtorrent tests (28/41 wrongful successes as of 03/28/2025)

# License

GNU AGPL v3
