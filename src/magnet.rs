use fluent_uri::pct_enc::{encoder::Query, EStr};
use fluent_uri::{ParseError as UriParseError, Uri};

use crate::{InfoHash, InfoHashError, TorrentID};

use std::string::FromUtf8Error;

/// Error occurred during parsing a [`MagnetLink`](crate::magnet::MagnetLink).
#[derive(Clone, Debug, PartialEq)]
pub enum MagnetLinkError {
    /// The URI was not valid according to [`Uri::parse`](fluent_uri::Uri::parse).
    InvalidURI { source: UriParseError },
    /// The URI does not contain a query.
    InvalidURINoQuery,
    /// The URI query contains non-UTF8 chars
    InvalidURIQueryUnicode { source: FromUtf8Error },
    /// The URI query contains a key without a value
    InvalidURIQueryEmptyValue { key: String },
    /// The URI query contains a non-urlencoded `?` beyond the query declaration
    InvalidURIQueryInterrogation,
    /// The URI contains a newline
    InvalidURINewLine,
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
    /// There were two or more `dn` declarations in the magnet query.
    DuplicateName,
    /// No name was contained in the magnet URI. This is technically allowed by
    /// some implementations, but should not be encouraged/supported.
    #[cfg(feature = "magnet_force_name")]
    NoNameFound,
}

impl std::fmt::Display for MagnetLinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MagnetLinkError::InvalidURI { source } => {
                write!(f, "Invalid URI: {source}")
            }
            MagnetLinkError::InvalidURINoQuery => {
                write!(f, "Invalid URI: no query string")
            }
            MagnetLinkError::InvalidURIQueryEmptyValue { key } => {
                write!(f, "Invalid URI: query has key {key} with no value")
            }
            MagnetLinkError::InvalidURIQueryUnicode { .. } => {
                write!(f, "Invalid URI: the query part contains non-utf8 chars")
            }
            MagnetLinkError::InvalidURIQueryInterrogation => {
                write!(f, "Invalid URI: the query part should only contain one `?`")
            }
            MagnetLinkError::InvalidURINewLine => {
                write!(f, "Invalid URI: newlines are not allowed in magnet links")
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
            MagnetLinkError::DuplicateName => {
                write!(
                    f,
                    "Too many name declarations for the magnet, only expecting one."
                )
            }
            #[cfg(feature = "magnet_force_name")]
            MagnetLinkError::NoNameFound => {
                write!(f, "No name found")
            }
        }
    }
}

impl From<InfoHashError> for MagnetLinkError {
    fn from(e: InfoHashError) -> MagnetLinkError {
        MagnetLinkError::InvalidHash { source: e }
    }
}

impl<Input> From<(UriParseError, Input)> for MagnetLinkError {
    fn from(e: (UriParseError, Input)) -> MagnetLinkError {
        MagnetLinkError::InvalidURI { source: e.0 }
    }
}

impl From<FromUtf8Error> for MagnetLinkError {
    fn from(e: FromUtf8Error) -> MagnetLinkError {
        MagnetLinkError::InvalidURIQueryUnicode { source: e }
    }
}

impl std::error::Error for MagnetLinkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MagnetLinkError::InvalidURI { source } => Some(source),
            MagnetLinkError::InvalidHash { source } => Some(source),
            // MagnetLinkError::InvalidURIQueryUnicode { source } => Some(source),
            _ => None,
        }
    }
}

/// A Magnet URI, which contains the infohash(es) but not the entire meta info.
///
/// The MagnetLink can provide information about the torrent
/// [`name`](crate::magnet::MagnetLink::name) and [`hash`](crate::magnet::MagnetLink::hash).
///
/// More information is specified in [BEP-0009](https://bittorrent.org/beps/bep_0009.html), and
/// even more appears in the wild, as explained [on Wikipedia](https://en.wikipedia.org/wiki/Magnet_URI_scheme).
#[derive(Clone, Debug)]
pub struct MagnetLink {
    /// Only mandatory field for magnet link parsing, unless the
    /// `magnet_force_name` crate feature is enabled.
    hash: InfoHash,
    /// Original query string from which the magnet was parsed.
    /// Used to format the magnet link back to a string.
    query: String,
    /// Name of the torrent, which may be empty unless
    /// `magnet_force_name` crate feature is enabled.
    name: String,
}

impl MagnetLink {
    /// Generates a new MagnetLink from a string. Will fail if the string is not a valid URL, and
    /// in the conditions defined in [`MagnetLink::from_url`](crate::magnet::MagnetLink::from_url).
    pub fn new(s: &str) -> Result<MagnetLink, MagnetLinkError> {
        // The error returned by Uri::parse when there is a newline is not very obvious, so we
        // sacrifice performance to save neurons from fellow developers.
        if s.contains('\n') {
            return Err(MagnetLinkError::InvalidURINewLine);
        }

        let u = Uri::parse(s.to_string())?;
        MagnetLink::from_url(&u)
    }

    /// Generates a new MagnetLink from a parsed URL.
    /// Will generate a weird name if multiple "dn" params are contained in the URL.
    /// Will fail if:
    ///   - the scheme is not `magnet`
    ///   - there is no name (`dn` URL param)
    ///   - no hash was found (`xt` URL param, with `urn:btih:` prefix for v1 infohash,
    ///     `urn:btmh:1220` for v2 infohash)
    ///   - more than one hash of the same type was found
    ///   - the hashes were not valid according to [`InfoHash::new`](crate::hash::InfoHash::new)
    pub fn from_url(u: &Uri<String>) -> Result<MagnetLink, MagnetLinkError> {
        if u.scheme().as_str() != "magnet" {
            return Err(MagnetLinkError::InvalidScheme {
                scheme: u.scheme().to_string(),
            });
        }

        let mut name = String::new();
        let mut hashes: Vec<String> = Vec::new();

        let query = u.query().ok_or(MagnetLinkError::InvalidURINoQuery)?;
        for (key, val) in Self::unsafe_parse_query(query)? {
            // magnets should not allow unescaped ? in query value
            if val.as_str().contains('?') {
                return Err(MagnetLinkError::InvalidURIQueryInterrogation);
            }

            // magnets should not allow empty query values
            if val.is_empty() {
                return Err(MagnetLinkError::InvalidURIQueryEmptyValue {
                    key: key.as_str().to_string(),
                });
            }

            match key.as_str() {
                "xt" => {
                    let val = val.as_str();
                    if val.starts_with("urn:btih:") {
                        // Infohash v1
                        hashes.push(val.strip_prefix("urn:btih:").unwrap().to_string());
                    } else if val.starts_with("urn:btmh:1220") {
                        // Infohash v2
                        hashes.push(val.strip_prefix("urn:btmh:1220").unwrap().to_string());
                    }
                }
                "dn" => {
                    if !name.is_empty() {
                        return Err(MagnetLinkError::DuplicateName);
                    }
                    name = val
                        .decode()
                        .into_string()?
                        // fluent_uri explicitly does not decode U+002B (`+`) as a space
                        .replace('+', " ")
                        .to_owned();
                }
                "tr" => {
                    // TODO: trackers
                }
                _ => {
                    continue;
                }
            }
        }

        #[cfg(feature = "magnet_force_name")]
        if name.is_empty() {
            return Err(MagnetLinkError::NoNameFound);
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
            valid_hashes.first().unwrap().clone()
        } else {
            let (hash1, hash2) = (valid_hashes.first().unwrap(), valid_hashes.get(1).unwrap());
            hash1.hybrid(hash2)?
        };

        Ok(MagnetLink {
            hash: final_hash,
            name: name.to_string(),
            query: query.as_str().to_string(),
        })
    }

    /// Parse the query in a list of key->value entries with a percent-decoder attached.
    ///
    /// The results can be accessed raw with [EStr::as_str()] and percent-decoded with [EStr::decode].
    ///
    /// This method only fails if the magnet query is empty (`magnet:`), but may produce unexpected
    /// results because it does not apply magnet-specific sanitation.
    ///
    /// This method has a dangerous-sounding name because of percent-encoding.
    /// If you aren't careful, you may end up with garbage data. This method
    /// is not actually memory-unsafe.
    ///
    /// For example:
    ///
    /// - a key without a value may be returned
    /// - duplicate entries may be returned (such as a double magnet name)
    /// - a value with an unencoded `?` may be returned
    #[allow(clippy::type_complexity)]
    pub fn unsafe_parse_query(
        query: &EStr<Query>,
    ) -> Result<Vec<(&EStr<Query>, &EStr<Query>)>, MagnetLinkError> {
        let pairs: Vec<(&EStr<Query>, &EStr<Query>)> = query
            .split('&')
            .map(|s| s.split_once('=').unwrap_or((s, EStr::EMPTY)))
            .collect();

        Ok(pairs)
    }

    /// Returns the [`InfoHash`](crate::hash::InfoHash) contained in the MagnetLink
    pub fn hash(&self) -> &InfoHash {
        &self.hash
    }

    /// Returns the torrent name contained in the MagnetLink. If multiple names are contained in the URL,
    /// they will all be appended. If no name is contained in the magnet link, the result of this function will be empty.
    /// However, when the `magnet_force_name` feature is enabled, the `MagnetLink` creation will have errored when the name
    /// is not provided and so this function is guaranteed to return an actual name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the [`TorrentID`](crate::id::TorrentID) for the MagnetLink
    pub fn id(&self) -> TorrentID {
        self.hash.id()
    }
}

impl std::fmt::Display for MagnetLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "magnet:?{}", self.query)
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
    #[cfg(not(feature = "magnet_force_name"))]
    fn can_load_without_name() {
        let magnet =
            MagnetLink::new("magnet:?xt=urn:btih:c811b41641a09d192b8ed81b14064fff55d85ce3")
                .unwrap();
        assert_eq!(magnet.name, "".to_string());
        assert_eq!(
            magnet.hash,
            InfoHash::V1("c811b41641a09d192b8ed81b14064fff55d85ce3".to_string())
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

    #[test]
    fn fails_newline_in_magnet() {
        let mut magnet_url = std::fs::read_to_string("tests/bittorrent-v2-test.magnet").unwrap();
        magnet_url.push('\n');

        let res = MagnetLink::new(&magnet_url);
        assert!(res.is_err());

        assert_eq!(res.unwrap_err(), MagnetLinkError::InvalidURINewLine,);
    }

    #[test]
    fn survives_roundtrip() {
        // Here we test that parsing a magnet then displaying it again
        // will produce exactly the same output.
        let magnet_url =
            Uri::parse(std::fs::read_to_string("tests/bittorrent-v2-test.magnet").unwrap())
                .unwrap();
        let magnet = MagnetLink::from_url(&magnet_url).unwrap();

        let magnet_str = magnet.to_string();
        assert_eq!(&magnet_url.to_string(), &magnet_str);
    }
}
