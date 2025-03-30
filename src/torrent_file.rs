use bt_bencode::Value as BencodeValue;
use rustc_hex::ToHex;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

use std::collections::HashMap;
use std::path::PathBuf;

use crate::{InfoHash, InfoHashError, PieceLength, TorrentContent, TorrentID};

/// Error occurred during parsing a [`TorrentFile`](crate::torrent_file::TorrentFile).
#[derive(Clone, Debug, PartialEq)]
pub enum TorrentFileError {
    NoNameFound,
    // TODO: bt_bencode::Error is not PartialEq so we store error as String
    InvalidBencode { reason: String },
    NotATorrent { reason: String },
    WrongVersion { version: u64 },
    InvalidHash { source: InfoHashError },
    InvalidContentPath { path: String },
    MissingPieceLength,
    BadPieceLength { piece_length: u32 },
}

impl std::fmt::Display for TorrentFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TorrentFileError::NoNameFound => write!(f, "No name found"),
            TorrentFileError::InvalidBencode { reason } => write!(f, "Invalid bencode: {reason}"),
            TorrentFileError::NotATorrent { reason } => write!(
                f,
                "Valid bencode, but does not seem to be a torrent ({reason})"
            ),
            TorrentFileError::WrongVersion { version } => write!(
                f,
                "Wrong torrent version: {version}, only v1 and v2 are supported)"
            ),
            TorrentFileError::InvalidHash { source } => write!(f, "Invalid hash: {source}"),
            TorrentFileError::InvalidContentPath { path } => {
                write!(f, "Invalid content file path in torrent: {path}")
            }
            TorrentFileError::MissingPieceLength => {
                write!(f, "No \'piece length\' field found in info dict")
            }
            TorrentFileError::BadPieceLength { piece_length } => {
                write!(f, "Torrent \'piece length\' is too big: {}", piece_length)
            }
        }
    }
}

impl From<InfoHashError> for TorrentFileError {
    fn from(e: InfoHashError) -> TorrentFileError {
        TorrentFileError::InvalidHash { source: e }
    }
}

impl From<bt_bencode::Error> for TorrentFileError {
    fn from(e: bt_bencode::Error) -> TorrentFileError {
        TorrentFileError::InvalidBencode {
            reason: e.to_string(),
        }
    }
}

impl std::error::Error for TorrentFileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TorrentFileError::InvalidHash { source } => Some(source),
            _ => None,
        }
    }
}

/// A torrent file.
///
/// The torrent file specification and related extensions are described on [Wikipedia](https://en.wikipedia.org/wiki/Torrent_file).
/// The TorrentFile can provide information about the torrent
/// [`name`](crate::torrent_file::TorrentFile::name) and
/// [`hash`](crate::torrent_file::TorrentFile::hash). Other fields could be supported, but are not
/// currently implemented by this library.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TorrentFile {
    pub hash: InfoHash,
    pub name: String,
    // Kept for further analysis
    pub decoded: DecodedTorrent,
}

/// A parsed bencode-decoded value, to ensure torrent-like structure.
///
/// In its present form, DecodedTorrent only cares about the info dict, but preserves other fields
/// as [`BencodeValue`](bt_bencode::BencodeValue) in an `extra` mapping so you can implement
/// your own extra parsing.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecodedTorrent {
    info: DecodedInfo,

    // Rest of torrent dict
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    extra: HashMap<String, BencodeValue>,
}

impl DecodedTorrent {
    pub fn files(&self) -> Result<Vec<TorrentContent>, TorrentFileError> {
        if self.info.files.is_none() {
            if self.info.file_tree.is_none() {
                // V1 torrent with single file
                Ok(vec![TorrentContent {
                    path: PathBuf::from(&self.info.name),
                    size: self.info.length.unwrap(),
                }])
            } else {
                todo!("v2 torrent files");
            }
        } else {
            // V1 torrent with multiple files
            let mut files: Vec<TorrentContent> = vec![];
            for file in self.info.files.as_ref().unwrap() {
                // TODO: error
                let f: UnsafeV1FileContent = bt_bencode::from_value(file.clone()).unwrap();
                if let Some(parsed_file) = f.to_torrent_content()? {
                    files.push(parsed_file);
                }
            }

            // Sort files by alphabetical order
            files.sort();
            Ok(files)
        }
    }
}

/// Raw file path described within a Bittorrent v1 torrent file.
///
/// It has not been sanitized, for example to prevent path traversal attacks. You should not be using this in your API;
/// use [TorrentContent] instead.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct UnsafeV1FileContent {
    /// Raw path segments from the torrent, may contain directory escapes (like `..`)
    #[serde(rename = "path")]
    pub raw_paths: Vec<String>,
    /// File length in bytes
    pub length: u64,
    /// Extended file attributes as defined in [BEP-0047](https://www.bittorrent.org/beps/bep_0047.html)
    ///
    /// Can contain several characters:
    ///
    /// - p for padding files
    /// - l for symlinks
    /// - x for executables
    /// - h for hidden files
    #[serde(default)]
    pub attr: String,
}

impl UnsafeV1FileContent {
    /// Tries to parse [TorrentContent].
    ///
    /// Fails if the data is invalid (eg. path traversal), produces
    /// Ok(None) when the file is a padding file.
    pub fn to_torrent_content(&self) -> Result<Option<TorrentContent>, TorrentFileError> {
        if self.attr.contains('p') {
            return Ok(None);
        }

        // Parse the raw path parts omitting weird directory shenanigans
        let mut path = PathBuf::new();
        for p in &self.raw_paths {
            if p.contains('/') {
                return Err(TorrentFileError::InvalidContentPath {
                    path: p.to_string(),
                });
            }

            if p == ".." {
                return Err(TorrentFileError::InvalidContentPath {
                    path: p.to_string(),
                });
            }

            if p == "." {
                continue;
            }

            path.push(p);
        }

        Ok(Some(TorrentContent {
            path,
            size: self.length,
        }))
    }
}

/// An info dict contained in a [`DecodedTorrent`](crate::torrent_file::DecodedTorrent).
///
/// Only cares about torrent version, name, and files, but other fields are preseved in an `extra`
/// mapping so you can implement your own extra parsing.
// bt_bencode does not support serializing None options and empty HashMaps, so we skip
// serialization in those cases.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecodedInfo {
    #[serde(rename = "meta version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<u64>,

    name: String,

    /// Torrent `piece length` as used in v1/v2 torrents
    #[serde(rename = "piece length")]
    piece_length: PieceLength,

    // Torrent v1/hybrid (only for single-file torrents)
    #[serde(skip_serializing_if = "Option::is_none")]
    length: Option<u64>,

    // Torrent v1 (only for multi-files torrents)
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<Vec<BencodeValue>>,

    // Torrent v2 (for both single and multi-files torrents)
    #[serde(rename = "file tree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    file_tree: Option<BencodeValue>,

    // Rest of info dict that we keep for hashing
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    extra: HashMap<String, BencodeValue>,
}

impl TorrentFile {
    pub fn from_slice(s: &[u8]) -> Result<TorrentFile, TorrentFileError> {
        let torrent: DecodedTorrent = bt_bencode::from_slice(s).map_err(|e| {
            // We store a stringy representation of the error because bt_encode::Error
            // is not PartialEq
            TorrentFileError::NotATorrent {
                reason: e.to_string(),
            }
        })?;

        // We just deserialized successfully so this is a safe unwrap
        // Unless we added an Option/HashMap and forgot to skip serialization when empty
        let info_bytes = bt_bencode::to_vec(&torrent.info).unwrap();

        let infohash = match torrent.info.version {
            // Most v1 torrents don't declare a torrent version at all
            Some(1) | None => {
                // Bittorrent v1 does not necessarily have a files dict... single-file torrents
                // just use the torrent name field for that
                let digest = Sha1::digest(&info_bytes).to_vec().to_hex::<String>();
                InfoHash::new(&digest)?
            }
            Some(2) => {
                // Bittorrent v2 has mandatory file_tree dict
                // see http://bittorrent.org/beps/bep_0052.html
                if torrent.info.file_tree.is_some() {
                    let digest = sha256::digest(info_bytes.as_slice());
                    let hash = InfoHash::new(&digest)?;
                    // Check if we have hybrid torrent...
                    // If it's single-file it will have length field
                    // If it's multi-file it will have files field
                    if torrent.info.length.is_some() || torrent.info.files.is_some() {
                        let digest = Sha1::digest(&info_bytes).to_vec().to_hex::<String>();
                        hash.hybrid(&InfoHash::new(&digest)?)?
                    } else {
                        hash
                    }
                } else {
                    return Err(TorrentFileError::NotATorrent {
                        reason: "Torrentv2 without 'file_tree' field".to_string(),
                    });
                }
            }
            _ => {
                // Version is not null and is not 1-2
                return Err(TorrentFileError::WrongVersion {
                    version: torrent.info.version.unwrap(),
                });
            }
        };

        Ok(TorrentFile {
            name: torrent.info.name.clone(),
            hash: infohash,
            decoded: torrent,
        })
    }

    pub fn hash(&self) -> &str {
        self.hash.as_str()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> TorrentID {
        TorrentID::from_infohash(&self.hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_torrent_v1() {
        let slice = std::fs::read("tests/bittorrent-v1-emma-goldman.torrent").unwrap();
        let res = TorrentFile::from_slice(&slice);
        println!("{:?}", res);
        assert!(res.is_ok());
        let torrent = res.unwrap();
        assert_eq!(
            &torrent.name,
            "Goldman, Emma - Essential Works of Anarchism"
        );
        assert_eq!(
            torrent.hash,
            InfoHash::V1("c811b41641a09d192b8ed81b14064fff55d85ce3".to_string())
        );
        assert_eq!(torrent.decoded.files().unwrap().len(), 94);
    }

    #[test]
    fn can_read_torrent_v1_multifile() {
        let slice = std::fs::read("tests/libtorrent/good/sample.torrent").unwrap();
        let res = TorrentFile::from_slice(&slice);
        println!("{:?}", res);
        assert!(res.is_ok());
        let torrent = res.unwrap();
        assert_eq!(&torrent.name, "sample");
        assert_eq!(
            torrent.hash,
            InfoHash::V1("58d8d15a4eb3bd9afabc9cee2564f78192777edb".to_string())
        );
        assert_eq!(
            torrent.decoded.files().unwrap(),
            vec!(
                TorrentContent {
                    path: PathBuf::from("text_file.txt"),
                    size: 20,
                },
                TorrentContent {
                    path: PathBuf::from("text_file2.txt"),
                    size: 25,
                }
            ),
        );
    }

    #[test]
    fn can_read_torrent_v1_wrongpath() {
        let slice = std::fs::read("tests/libtorrent/good/parent_path.torrent").unwrap();
        let res = TorrentFile::from_slice(&slice);
        println!("{:?}", res);
        assert!(res.is_ok());
        let torrent = res.unwrap();
        assert_eq!(&torrent.name, "temp");
        assert_eq!(
            torrent.hash,
            InfoHash::V1("9e1111f1ee4966f7d06d398f1d58e00ad150657a".to_string())
        );
        assert_eq!(
            torrent.decoded.files().unwrap_err(),
            TorrentFileError::InvalidContentPath {
                path: "..".to_string()
            },
        );
    }

    #[test]
    fn can_read_torrent_v1_singlepath() {
        let slice = std::fs::read("tests/libtorrent/good/base.torrent").unwrap();
        let res = TorrentFile::from_slice(&slice);
        println!("{:?}", res);
        assert!(res.is_ok());
        let torrent = res.unwrap();
        assert_eq!(&torrent.name, "temp");
        assert_eq!(
            torrent.hash,
            InfoHash::V1("c0fda1edafdbdbb96443424e0b3899af7159d10e".to_string())
        );
        assert_eq!(
            torrent.decoded.files().unwrap(),
            vec!(TorrentContent {
                path: PathBuf::from("temp"),
                size: 425,
            }),
        );
    }

    #[test]
    fn can_read_torrent_v2() {
        let slice = std::fs::read("tests/bittorrent-v2-test.torrent").unwrap();
        let res = TorrentFile::from_slice(&slice);
        assert!(res.is_ok());
        let torrent = res.unwrap();
        assert_eq!(&torrent.name, "bittorrent-v2-test");
        assert_eq!(
            torrent.hash,
            InfoHash::V2(
                "caf1e1c30e81cb361b9ee167c4aa64228a7fa4fa9f6105232b28ad099f3a302e".to_string()
            )
        );
    }

    #[test]
    fn can_read_torrent_hybrid() {
        let slice = std::fs::read("tests/bittorrent-v2-hybrid-test.torrent").unwrap();
        let res = TorrentFile::from_slice(&slice);
        assert!(res.is_ok());
        let torrent = res.unwrap();
        assert_eq!(&torrent.name, "bittorrent-v1-v2-hybrid-test");
        assert_eq!(
            torrent.hash,
            InfoHash::Hybrid((
                "631a31dd0a46257d5078c0dee4e66e26f73e42ac".to_string(),
                "d8dd32ac93357c368556af3ac1d95c9d76bd0dff6fa9833ecdac3d53134efabb".to_string()
            ))
        );
    }

    #[test]
    fn v1_piece_len() {
        let slice = std::fs::read("tests/libtorrent/bad/negative_piece_len.torrent").unwrap();
        let res = TorrentFile::from_slice(&slice);
        assert!(res.is_err());
    }

    #[test]
    fn v2_piece_len() {
        let slice = std::fs::read("tests/libtorrent/bad/v2_piece_size.torrent").unwrap();
        let res = TorrentFile::from_slice(&slice);
        assert!(res.is_err());
    }
}
