use serde::{Deserialize, Serialize};

use std::str::FromStr;

use crate::{InfoHash, InfoHashError};

/// An infohash string truncated to 40 characters.
///
/// This representation is used by libtorrent, among others, for interoperability with software
/// that was meant to be 40-characters v1 infohashes. For v1 infohashes, the string representation
/// of the TorrentID is identical. For hybrid and v2 infohashes however, it is truncated to 40
/// characters.
///
/// A TorrentID can be generated from a string, and if it is not valid will return an
/// [`InfoHashError`](crate::hash::InfoHashError). It can also be generated from an actual
/// [`InfoHash`](crate::hash::InfoHash) with the
/// [`TorrentID::from_infohash`](crate::id::TorrentID::from_infohash) and
/// [`InfoHash::id`](crate::hash::InfoHash::id) methods.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TorrentID(String);

impl TorrentID {
    pub fn new<T: AsRef<str>>(s: T) -> Result<TorrentID, InfoHashError> {
        Self::from_str(s.as_ref())
    }

    pub fn from_infohash(hash: &InfoHash) -> TorrentID {
        match hash {
            InfoHash::V1(v2hash) => TorrentID(v2hash.to_string()),
            InfoHash::V2(v2hash) | InfoHash::Hybrid((_, v2hash)) => {
                let mut truncated = v2hash.to_string();
                truncated.truncate(40);
                TorrentID(truncated)
            }
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TorrentID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for TorrentID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl FromStr for TorrentID {
    type Err = InfoHashError;

    fn from_str(s: &str) -> Result<TorrentID, InfoHashError> {
        let hash = InfoHash::new(s)?;
        Ok(Self::from_infohash(&hash))
    }
}
