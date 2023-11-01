use url::Url;

use crate::{InfoHash, InfoHashError, TorrentID};

/// Error occurred during parsing a [`MagnetLink`](crate::magnet::MagnetLink).
#[derive(Clone, Debug, PartialEq)]
pub enum MagnetLinkError {
    /// The URI was not valid according to [`Url::parse`](url::Url::parse).
    InvalidURI { source: url::ParseError },
    /// The URI scheme was not `magnet`
    InvalidScheme { scheme: String },
    /// No Bittorrent v1/v2 hash was found in the magnet URI
    NoHashFound,
    /// A Bittorrent v1/v2 hash found in magnet URI was not a valid
    /// [`InfoHash`](crate::hash::InfoHash::new), or conflicting hashes were found
    /// (eg. two infohash v1 in the same URI).
    InvalidHash { source: InfoHashError },
    /// Too many hashes were found in the magnet URI, expected two at most.
    TooManyHashes { number: usize },
}

impl std::fmt::Display for MagnetLinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MagnetLinkError::InvalidURI { source } => {
                write!(f, "Invalid URI: {source}")
            }
            MagnetLinkError::InvalidScheme { scheme } => {
                write!(f, "Invalid URI scheme: {scheme}")
            }
            MagnetLinkError::NoHashFound => {
                write!(f, "No hash found (only btih/btmh hashes are supported)")
            }
            MagnetLinkError::InvalidHash { source } => {
                write!(f, "Invalid hash: {source}")
            }
            MagnetLinkError::TooManyHashes { number } => {
                write!(f, "Too many hashes ({number})")
            }
        }
    }
}

impl From<InfoHashError> for MagnetLinkError {
    fn from(e: InfoHashError) -> MagnetLinkError {
        MagnetLinkError::InvalidHash { source: e }
    }
}

impl From<url::ParseError> for MagnetLinkError {
    fn from(e: url::ParseError) -> MagnetLinkError {
        MagnetLinkError::InvalidURI { source: e }
    }
}

impl std::error::Error for MagnetLinkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MagnetLinkError::InvalidURI { source } => Some(source),
            MagnetLinkError::InvalidHash { source } => Some(source),
            _ => None,
        }
    }
}

/// A Magnet URI, which contains the infohash(es) but not the entire meta info.
///
/// The MagnetLink can provide information about the torrent
/// [`name`](crate::magnet::MagnetLink::name) and [`hash`](crate::magnet::MagnetLink::hash).
/// Other fields can be contained in the magnet URI, as explained [on Wikipedia](https://en.wikipedia.org/wiki/Magnet_URI_scheme). However,
/// they are currently not exposed by this library.
#[derive(Clone, Debug)]
pub struct MagnetLink {
    hash: InfoHash,
    name: String,
}

impl MagnetLink {
    /// Generates a new MagnetLink from a string. Will fail if the string is not a valid URL, and
    /// in the conditions defined in [`MagnetLink::from_url`](crate::magnet::MagnetLink::from_url).
    pub fn new(s: &str) -> Result<MagnetLink, MagnetLinkError> {
        let u = Url::parse(s)?;
        MagnetLink::from_url(&u)
    }

    /// Generates a new MagnetLink from a parsed URL.
    /// Will generate a weird name if multiple "dn" params are contained in the URL.
    /// Will fail if:
    ///   - the scheme is not `magnet`
    ///   - there is no name (`dn` URL param)
    ///   - no hash was found (`xt` URL param, with `urn:btih:` prefix for v1 infohash,
    ///   `urn:btmh:1220` for v2 infohash)
    ///   - more than one hash of the same type was found
    ///   - the hashes were not valid according to [`InfoHash::new`](crate::hash::InfoHash::new)
    pub fn from_url(u: &Url) -> Result<MagnetLink, MagnetLinkError> {
        if u.scheme() != "magnet" {
            return Err(MagnetLinkError::InvalidScheme {
                scheme: u.scheme().to_string(),
            });
        }

        let mut name = String::new();
        let mut hashes: Vec<String> = Vec::new();

        for (key, val) in u.query_pairs() {
            // Deref cow into str then reference it
            match &*key {
                "xt" => {
                    if val.starts_with("urn:btih:") {
                        // Infohash v1
                        hashes.push(val.strip_prefix("urn:btih:").unwrap().to_string());
                    } else if val.starts_with("urn:btmh:1220") {
                        // Infohash v2
                        hashes.push(val.strip_prefix("urn:btmh:1220").unwrap().to_string());
                    }
                }
                "dn" => {
                    name.push_str(&val);
                }
                _ => continue,
            }
        }

        let hashes_len = hashes.len();

        if hashes_len == 0 {
            return Err(MagnetLinkError::NoHashFound);
        }

        if hashes_len > 2 {
            return Err(MagnetLinkError::TooManyHashes { number: hashes_len });
        }

        // Check hashes sanity
        let mut valid_hashes: Vec<InfoHash> = Vec::new();
        for hash in hashes {
            let valid_hash = InfoHash::new(&hash)?;
            valid_hashes.push(valid_hash);
        }

        // If we still have two hashes not just one, we should combine them into hybrid
        // Otherwise we just return the first and only infohash found
        let final_hash = if valid_hashes.len() == 1 {
            valid_hashes.get(0).unwrap().clone()
        } else {
            let (hash1, hash2) = (valid_hashes.get(0).unwrap(), valid_hashes.get(1).unwrap());
            hash1.hybrid(hash2)?
        };

        Ok(MagnetLink {
            name,
            hash: final_hash,
        })
    }

    /// Returns the [`InfoHash`](crate::hash::InfoHash) contained in the MagnetLink
    pub fn hash(&self) -> &InfoHash {
        &self.hash
    }

    /// Returns the torrent name contained in the MagnetLink. If multiple names are contained in the URL,
    /// they will all be appended.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the [`TorrentID`](crate::id::TorrentID) for the MagnetLink
    pub fn id(&self) -> TorrentID {
        self.hash.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_v1() {
        let magnet_source =
            std::fs::read_to_string("tests/bittorrent-v1-emma-goldman.magnet").unwrap();
        let magnet = MagnetLink::new(&magnet_source).unwrap();
        assert_eq!(
            magnet.name,
            "Emma Goldman - Essential Works of Anarchism (16 books)".to_string()
        );
        assert_eq!(
            magnet.hash,
            InfoHash::V1("c811b41641a09d192b8ed81b14064fff55d85ce3".to_string())
        );
    }

    #[test]
    fn can_load_hybrid() {
        let magnet_source =
            std::fs::read_to_string("tests/bittorrent-v2-hybrid-test.magnet").unwrap();
        let magnet = MagnetLink::new(&magnet_source).unwrap();
        assert_eq!(magnet.name, "bittorrent-v1-v2-hybrid-test");
        assert_eq!(
            magnet.hash,
            InfoHash::Hybrid((
                "631a31dd0a46257d5078c0dee4e66e26f73e42ac".to_string(),
                "d8dd32ac93357c368556af3ac1d95c9d76bd0dff6fa9833ecdac3d53134efabb".to_string()
            ))
        );
    }

    #[test]
    fn can_load_v2() {
        let magnet_source = std::fs::read_to_string("tests/bittorrent-v2-test.magnet").unwrap();
        let magnet = MagnetLink::new(&magnet_source).unwrap();
        assert_eq!(magnet.name, "bittorrent-v2-test".to_string());
        assert_eq!(
            magnet.hash,
            InfoHash::V2(
                "caf1e1c30e81cb361b9ee167c4aa64228a7fa4fa9f6105232b28ad099f3a302e".to_string()
            )
        );
    }

    #[test]
    fn can_load_without_name() {
        let magnet = MagnetLink::new("magnet:?xt=urn:btih:c811b41641a09d192b8ed81b14064fff55d85ce3").unwrap();
        assert_eq!(magnet.name, "".to_string());
        assert_eq!(
            magnet.hash,
            InfoHash::V1(
                "c811b41641a09d192b8ed81b14064fff55d85ce3".to_string()
            )
        );
    }

    #[test]
    fn fails_load_no_hash() {
        let res = MagnetLink::new(
            "magnet:?dn=Goldman%2c%20Emma%20-%20Essential%20Works%20of%20Anarchism",
        );
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, MagnetLinkError::NoHashFound);
    }

    #[test]
    fn fails_load_too_many_hashes() {
        let res = MagnetLink::new("magnet:?xt=urn:btih:c811b41641a09d192b8ed81b14064fff55d85ce3&dn=Goldman%2c%20Emma%20-%20Essential%20Works%20of%20Anarchism&xt=urn:btih:c811b41641a09d192b8ed81b14064fff55d85ce4&xt=urn:btih:c811b41641a09d192b8ed81b14064fff55d85ce5");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, MagnetLinkError::TooManyHashes { number: 3 });
    }

    #[test]
    fn fails_load_conflicting_hash() {
        let res = MagnetLink::new("magnet:?xt=urn:btih:c811b41641a09d192b8ed81b14064fff55d85ce3&dn=Goldman%2c%20Emma%20-%20Essential%20Works%20of%20Anarchism&xt=urn:btih:c811b41641a09d192b8ed81b14064fff55d85ce4");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(
            err,
            MagnetLinkError::InvalidHash {
                source: InfoHashError::FailedHybrid {
                    hashtype: "V1".to_string()
                }
            }
        );
    }

    #[test]
    fn fails_load_illegal_uri_chars() {
        let res = MagnetLink::new("magnet:?xt=urn:btih:c811b41641a09d192b8ed81b14064fff55d85ce3&dn=Goldman%2c%20Emma%20-%20Essential%20Works%20of%20Anarchism&xt=urn:btih:c811b41641a09d192b8ed81b14064fff55d85ce4&xt=urn:btih:c811b41641a09d192b8ed81b14064fff55d85ce5");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, MagnetLinkError::TooManyHashes { number: 3 });
    }

    #[test]
    fn fails_load_invalid_hash_chars() {
        let res = MagnetLink::new("magnet:?xt=urn:btih:c811b41641a09d192b8ed81b14064fff55d85WWW&dn=Goldman%2c%20Emma%20-%20Essential%20Works%20of%20Anarchism");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(
            err,
            MagnetLinkError::InvalidHash {
                source: InfoHashError::InvalidChars {
                    hash: "c811b41641a09d192b8ed81b14064fff55d85WWW".to_string()
                }
            }
        );
    }

    #[test]
    fn fails_load_invalid_hash_length() {
        let res = MagnetLink::new("magnet:?xt=urn:btih:c811b41641a09d192b8ed81b14064fff55d85ce311&dn=Goldman%2c%20Emma%20-%20Essential%20Works%20of%20Anarchism");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(
            err,
            MagnetLinkError::InvalidHash {
                source: InfoHashError::InvalidLength {
                    len: 42,
                    hash: "c811b41641a09d192b8ed81b14064fff55d85ce311".to_string()
                }
            }
        );
    }

    #[test]
    fn fails_load_not_magnet() {
        let res = MagnetLink::new("https://fr.wikipedia.org");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(
            err,
            MagnetLinkError::InvalidScheme {
                scheme: "https".to_string()
            }
        );
    }
}
