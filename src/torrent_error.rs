#[derive(Debug, Snafu)]
#[snafu(context(suffix(Error)), visibility(pub))]
pub enum TorrentError {
    #[snafu(display("Invalid infohash: {source}"))]
    Hash {
        source: crate::info_hash::InfoHashError,
    },
    #[snafu(display("Invalid magnet: {source}"))]
    MagnetLink {
        source: crate::magnet_link::MagnetLinkError,
    },
    TorrentFile {
        source: crate::torrent_file::TorrentFileError,
    },
    // TODO: deprecate below?

    //    #[snafu(display("Invalid torrent file {path}:\n{source}"))]
    //    InvalidTorrent { path: PathBuf, source: bt_bencode::Error },
    #[snafu(display("Invalid magnet link"))]
    InvalidMagnet,
    #[snafu(display("Missing magnet hash type"))]
    EmptyHashType,
    #[snafu(display("Invalid magnet hashtype: {hash_type}"))]
    InvalidMagnetHashType { hash_type: String },
    #[snafu(display("Missing magnet hash"))]
    EmptyHash,
    //#[snafu(display("Invalid magnet hash of type {hash_type}: {hash}"))]
    //InvalidMagnetHash { hash_type: String, hash: String },
    #[snafu(display("Missing magnet name"))]
    EmptyName,
    #[snafu(display("Invalid bencode for torrent file"))]
    InvalidBencode { source: bt_bencode::Error },
    #[snafu(display("Torrent has no info section"))]
    EmptyInfo,
    #[snafu(display("Missing torrent name"))]
    EmptyTorrentName,
    #[snafu(display("Wrong torrent version number: {version}"))]
    WrongTorrentVersion { version: u64 },
    #[snafu(display("The following hash contains non-hex characters: {hash}"))]
    InvalidHashChar { hash: String },
    #[snafu(display("The following hash has a wrong length (not 40/64 bytes): {hash}"))]
    InvalidHashLength { hash: String },
    #[snafu(display("Invalid magnet hash {hash_type}:{hash}"))]
    InvalidMagnetHash { hash: String, hash_type: String },
    #[snafu(display(
        "The advertised magnet hash type {hash_type} mismatched the actual hash: {hash}"
    ))]
    MismatchedMagnetHashType { hash: String, hash_type: String },
    #[snafu(display("Unsupported magnet hash type: {}", hash_type))]
    UnsupportedMagnetHashType { hash_type: String },
}

impl From<crate::info_hash::InfoHashError> for TorrentError {
    fn from(e: crate::info_hash::InfoHashError) -> TorrentError {
        TorrentError::Hash { source: e }
    }
}
