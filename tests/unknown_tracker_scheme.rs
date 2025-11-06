use hightorrent::{InfoHash, TorrentContent, TorrentFile};

use std::path::PathBuf;

#[test]
fn can_parse_no_scheme_tracker() {
    // This only works when the unknown_tracker_scheme crate feature is eanbled
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
