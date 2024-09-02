use hightorrent::{MagnetLink, MagnetLinkError};

#[test]
fn fails_load_no_name() {
    let res = MagnetLink::new("magnet:?xt=urn:btih:c811b41641a09d192b8ed81b14064fff55d85ce3");
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err, MagnetLinkError::NoNameFound);
}
