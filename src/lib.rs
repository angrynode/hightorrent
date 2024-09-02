//! HighTorrent is a library which contains high-level data structures and functions
//! to interact with Bittorrent v1 and v2 torrents. HighTorrent does not aim to be featureful,
//! but rather to be super easy to use and interoperate with more advanced torrent software,
//! and completely impossible to misuse.
//!
//! **Note that HighTorrent is not a networked library. It will not provide any utilities for
//! querying the DHT and/or downloading torrents. HighTorrent is much lower in the stack.**
//!
//! HighTorrent provides utilities to extract name and hash from torrents/magnets, using the
//! [`MagnetLink`](crate::magnet::MagnetLink) and [`TorrentFile`](crate::torrent_file::TorrentFile) structures, but could provide more advanced utilities in the future (PRs welcome). Additionally, it provides the [`Torrent`](crate::torrent::Torrent) struct and the
//! [`IntoTorrent`](crate::torrent::IntoTorrent) trait representing fully-loaded torrents ; those helpers are intended to be used by more diverse torrenting libraries to provide interoperability out-of-the-box.
//!
//! Finally, the [`SingleTarget`](crate::target::SingleTarget) and
//! [`MultiTarget`](crate::target::MultiTarget) structures represent one or more torrents you wish to
//! interact with. The contained stringy value is ambiguous, and can represent either a precise
//! [`InfoHash`](crate::hash::InfoHash) or a libtorrent-compatible [`TorrentID`](crate::id::TorrentID) (truncated hash).

#[macro_use]
extern crate serde;

mod hash;
pub use hash::{InfoHash, InfoHashError, TryInfoHash};

mod id;
pub use id::TorrentID;

mod list;
pub use list::TorrentList;

mod magnet;
pub use magnet::{MagnetLink, MagnetLinkError};

mod torrent;
pub use torrent::{ToTorrent, Torrent};

mod torrent_file;
pub use torrent_file::{TorrentFile, TorrentFileError};

mod target;
pub use target::{MultiTarget, SingleTarget, ToSingleTarget};

mod tracker;
pub use tracker::{PeerSource, Tracker, TrackerError, TrackerScheme, TryIntoTracker};
