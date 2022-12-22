use serde::{Serialize, Deserialize};

use std::str::FromStr;

use crate::TorrentID;

/// Error occurred during parsing a [`InfoHash`](crate::hash::InfoHash).
#[derive(Clone, Debug, PartialEq)]
pub enum InfoHashError {
    InvalidChars { hash: String },
    InvalidLength { hash: String, len: usize },
    FailedHybrid { hashtype: String },
    CannotHybridHybrid,
}

impl std::fmt::Display for InfoHashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfoHashError::InvalidChars { hash } => {
                write!(f, "Hash contains non-hex characters: {hash}")
            }, InfoHashError::InvalidLength { hash, len } => {
                write!(f, "Hash has invalid length {len} (expected 40 or 64): {hash}")
            }, InfoHashError::FailedHybrid { hashtype } => {
                write!(f, "Cannot make hybrid out of two {hashtype} hashes (same hash types)")
            }, InfoHashError::CannotHybridHybrid => {
                write!(f, "Cannot make a hybrid out of an already-hybrid infohash")
            }
        }
    }
}

impl std::error::Error for InfoHashError {}

/// A torrent's infohash, represented by a stringy lowercase hexadecimal digest.
///
/// The [`InfoHash`] can be either
/// a Bittorrent v1 info hash (40 chars sha1) or Bittorrent v2 info hash (64 chars sha256). In both cases, the hash
/// is guaranteed to be a valid sha1/sha256 lowercase hex digest and not a random string.
/// Alternatively, the Hybrid variant holds both v1 and v2 lowercase hex digests.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InfoHash {
    V1(String),
    V2(String),
    Hybrid((String, String)),
}

impl InfoHash {
    /// Generates an InfoHash from a string.
    ///
    /// Will fail if the string contains non-hexadecimal characters, or if its length is not
    /// exactly 40 or 64 characters. This method should only be used for generating an actual InfoHash.
    ///
    /// If you want to match a torrent by user-submitted input, where a Bittorrent v1 digest and a
    /// truncated Bittorrent v2 are ambiguous, you should use [SingleTarget](crate::target::SingleTarget)
    /// instead. If you want to unambiguously designate a Torrent by a 40 characters identifier,
    /// you should use [`TorrentID`](crate::id::TorrentID) instead.
    pub fn new(hash: &str) -> Result<InfoHash, InfoHashError> {
        if !hash.as_bytes().iter().all(|b| b.is_ascii_hexdigit()) {
            return Err(InfoHashError::InvalidChars { hash: hash.to_string() })
        }

        let hash = hash.to_lowercase();
        let len = hash.len();

        if len == 40 {
            Ok(InfoHash::V1(hash))
        } else if len == 64 {
            Ok(InfoHash::V2(hash))
        } else {
            Err(InfoHashError::InvalidLength { hash: hash.to_string(), len })
        }
    }

    /// Takes the current infohash and hybrids it with a second infohash.
    /// Returns an error if the two hash types are identical.
    pub fn hybrid(&self, with: &InfoHash) -> Result<InfoHash, InfoHashError> {
        match (&self, &with) {
            (&InfoHash::V1(hash1), &InfoHash::V2(hash2)) => Ok(InfoHash::Hybrid((hash1.to_string(), hash2.to_string()))),
            (&InfoHash::V2(hash2), &InfoHash::V1(hash1)) => Ok(InfoHash::Hybrid((hash1.to_string(), hash2.to_string()))),
            (&InfoHash::V1(_), &InfoHash::V1(_)) => Err(InfoHashError::FailedHybrid { hashtype: "V1".to_string() }),
            (&InfoHash::V2(_), &InfoHash::V2(_)) => Err(InfoHashError::FailedHybrid { hashtype: "V2".to_string() }),
            _ => Err(InfoHashError::CannotHybridHybrid),
        }
    }

    /// Returns a stringy representation of the infohash. In case of an hybrid infohash, the v2
    /// hash is used.
    pub fn as_str(&self) -> &str {
        match &self {
            Self::V1(s) => &s,
            Self::V2(s) => &s,
            Self::Hybrid((_h1, h2)) => &h2,
        }
    }

    /// Returns a [`TorrentID`](crate::id::TorrentID) for the InfoHash. This is either the
    /// infohash v1, or the infohash v2 truncated to 40 characters for v2/hybrid infohash.
    pub fn id(&self) -> TorrentID {
        TorrentID::from_infohash(&self)
    }
}

impl std::fmt::Display for InfoHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for InfoHash {
    type Err = InfoHashError;

    fn from_str(s: &str) -> Result<InfoHash, Self::Err> {
        InfoHash::new(s)
    }
}

/// Try to turn a stringy value into an [InfoHash](crate::hash::InfoHash). For user-submitted data that may or may not be
/// actual infohash, use [ToSingleTarget](crate::target::ToSingleTarget) instead.
pub trait TryInfoHash {
    fn try_infohash(&self) -> Result<InfoHash, InfoHashError>;
}

impl<S> TryInfoHash for S where S: AsRef<str> {
    fn try_infohash(&self) -> Result<InfoHash, InfoHashError> {
        InfoHash::new(self.as_ref())
    }
}

impl TryInfoHash for InfoHash {
    fn try_infohash(&self) -> Result<InfoHash, InfoHashError> {
        Ok(self.clone())
    }
}

impl TryInfoHash for &InfoHash {
    fn try_infohash(&self) -> Result<InfoHash, InfoHashError> {
        Ok(InfoHash::new(self.as_str())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_infohash_v1() {
        let res = InfoHash::new("c811b41641a09d192b8ed81b14064fff55d85ce3");
        assert!(res.is_ok());
        let hash = res.unwrap();
        assert_eq!(hash, InfoHash::V1("c811b41641a09d192b8ed81b14064fff55d85ce3".to_string()));
    }

    #[test]
    fn can_load_infohash_v2() {
        let res = InfoHash::new("caf1e1c30e81cb361b9ee167c4aa64228a7fa4fa9f6105232b28ad099f3a302e");
        assert!(res.is_ok());
        let hash = res.unwrap();
        assert_eq!(hash, InfoHash::V2("caf1e1c30e81cb361b9ee167c4aa64228a7fa4fa9f6105232b28ad099f3a302e".to_string()));
    }

    #[test]
    fn does_lowercase_infohash() {
        let res = InfoHash::new("C811B41641A09D192B8eD81B14064FFF55D85CE3");
        assert!(res.is_ok());
        let hash = res.unwrap();
        assert_eq!(hash, InfoHash::V1("c811b41641a09d192b8ed81b14064fff55d85ce3".to_string()));
    }

    #[test]
    fn can_hybrid_v1_and_v2() {
        let hashv1 = InfoHash::new("c811b41641a09d192b8ed81b14064fff55d85ce3").unwrap();
        let hashv2 = InfoHash::new("caf1e1c30e81cb361b9ee167c4aa64228a7fa4fa9f6105232b28ad099f3a302e").unwrap();
        let res = hashv1.hybrid(&hashv2);
        assert!(res.is_ok());
        let hash = res.unwrap();
        assert_eq!(hash, InfoHash::Hybrid((
            "c811b41641a09d192b8ed81b14064fff55d85ce3".to_string(),
            "caf1e1c30e81cb361b9ee167c4aa64228a7fa4fa9f6105232b28ad099f3a302e".to_string()
        )));
    }

    #[test]
    fn fails_invalid_chars() {
        let res = InfoHash::new("D811B41641A09D192B8eD81B14064FFF55D85WWW");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, InfoHashError::InvalidChars { hash: "D811B41641A09D192B8eD81B14064FFF55D85WWW".to_string() });
    }

    #[test]
    fn fails_invalid_length() {
        let res = InfoHash::new("caf1e1c30e81cb361b9ee167c4aa64228a7fa4fa9f6105232b28ad099f3a302eAAAA");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, InfoHashError::InvalidLength { len: 68, hash: "caf1e1c30e81cb361b9ee167c4aa64228a7fa4fa9f6105232b28ad099f3a302eaaaa".to_string() });
    }

    #[test]
    fn fails_hybrid_conflicting_hashes() {
        let hash = InfoHash::new("caf1e1c30e81cb361b9ee167c4aa64228a7fa4fa9f6105232b28ad099f3a302e").unwrap();
        let res = hash.hybrid(&hash);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, InfoHashError::FailedHybrid { hashtype: "V2".to_string() });
    }

    #[test]
    fn failed_hybrid_hybrid() {
        let hashv1 = InfoHash::new("c811b41641a09d192b8ed81b14064fff55d85ce3").unwrap();
        let hashv2 = InfoHash::new("caf1e1c30e81cb361b9ee167c4aa64228a7fa4fa9f6105232b28ad099f3a302e").unwrap();
        let hybrid = hashv1.hybrid(&hashv2).unwrap();
        let res = hybrid.hybrid(&hashv2);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, InfoHashError::CannotHybridHybrid);
    }

    #[test]
    fn failed_empty_string() {
        let res = InfoHash::new("");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, InfoHashError::InvalidLength { hash: "".to_string(), len: 0 });
    }
}
