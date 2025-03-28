use serde::Deserialize;

use crate::{InfoHash, TorrentID};

use std::path::PathBuf;

/// Turn a backend-specific torrent into an agnostic [`Torrent`](crate::torrent::Torrent).
pub trait ToTorrent {
    fn to_torrent(&self) -> Torrent;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// An abstract torrent, loaded from any backend that implements
/// [ToTorrent](crate::torrent::ToTorrent).
pub struct Torrent {
    pub name: String,
    pub path: String,
    pub date_start: i64,
    pub date_end: i64,
    /// Progress percentage (0-100)
    pub progress: u8,
    pub size: i64,
    pub state: String,
    pub tags: Vec<String>,
    /// The infohash of this torrent
    pub hash: InfoHash,
    /// The libtorrent-compatible TorrentID
    /// v1 infohash is untouched, v2 infohash of the hybrid/v2 torrent is truncated to the first 40 chars
    pub id: TorrentID,
}

impl Torrent {
    /// This method is only used for tests. It will not have any useful information
    /// except for the hash and id.
    #[allow(dead_code)]
    pub(crate) fn dummy_from_hash(hash: &InfoHash) -> Torrent {
        Torrent {
            name: String::new(),
            path: String::new(),
            date_start: 0,
            date_end: 0,
            progress: 0,
            size: 0,
            state: String::new(),
            tags: Vec::new(),
            hash: hash.clone(),
            id: hash.id(),
        }
    }
}

pub trait ToTorrentContent {
    fn to_torrent_content(&self) -> TorrentContent;
}

/// A file contained inside a [Torrent].
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TorrentContent {
    /// File path, relative from the torrent root.
    pub path: PathBuf,
    /// Size of the file in bytes,
    pub size: u64,
}

impl std::cmp::PartialOrd for TorrentContent {
    // Sort by alphabetical order
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for TorrentContent {
    // Sort by alphabetical order
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path.cmp(&other.path)
    }
}
