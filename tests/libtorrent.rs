use hightorrent::TorrentFile;

use std::path::PathBuf;

#[test]
#[ignore]
fn respects_error_cases() {
    // The error cases defined by libtorrent
    let error_cases: Vec<PathBuf> = std::fs::read_dir("tests/libtorrent/bad")
        .unwrap()
        .map(|entry| entry.unwrap().path().to_path_buf())
        .collect();

    // Listing torrents that were wrongfully successful
    let mut missing_errors: Vec<PathBuf> = vec![];

    for case in &error_cases {
        let slice = std::fs::read(&case).unwrap();
        if let Ok(_) = TorrentFile::from_slice(&slice) {
            // Torrent was succesfully loaded, but should have failed
            missing_errors.push(case.to_path_buf());
        }
    }

    println!(
        "{} missing error cases (out of {}):",
        missing_errors.len(),
        error_cases.len()
    );
    for error in &missing_errors {
        println!("  - {}", error.display());
    }

    assert!(missing_errors.is_empty());
}
