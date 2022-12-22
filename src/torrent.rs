use serde::Deserialize;

use crate::{InfoHash, TorrentID};

/// Turn a backend-specific torrent into an agnostic [`Torrent`](crate::torrent::Torrent).
pub trait IntoTorrent {
    fn into_torrent(&self) -> Torrent;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// An abstract torrent, loaded from any backend that implements
/// [IntoTorrent](crate::torrent::IntoTorrent).
pub struct Torrent {
    //pub hash: TruncatedHash,
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
