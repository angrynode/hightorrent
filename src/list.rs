use crate::{SingleTarget, Torrent};

/// A list of [`Torrent`](crate::torrent::Torrent), with querying/filtering capabilities.
///
/// TODO: Implement filter method for finding MultipleTarget
#[derive(Clone, Serialize, Deserialize)]
pub struct TorrentList(Vec<Torrent>);

impl TorrentList {
    pub fn new() -> TorrentList {
        TorrentList(Vec::new())
    }

    pub fn push(&mut self, entry: Torrent) {
        self.0.push(entry);
    }

    pub fn from_vec(list: Vec<Torrent>) -> TorrentList {
        TorrentList(list)
    }

    pub fn to_vec(self) -> Vec<Torrent> {
        self.0
    }

    /// Find a single torrent in the TorrentList, matching a specific
    /// [`SingleTarget`](crate::target::SingleTarget).
    pub fn get(&self, target: &SingleTarget) -> Option<Torrent> {
        self.0
            .iter()
            .find(|t| target.matches_hash(&t.hash))
            .cloned()
    }
}

impl Default for TorrentList {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for TorrentList {
    type Item = Torrent;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Torrent> for TorrentList {
    fn from_iter<I: IntoIterator<Item = Torrent>>(iter: I) -> Self {
        let mut c = TorrentList::new();

        for i in iter {
            c.push(i);
        }

        c
    }
}

#[cfg(test)]
mod tests {
    use crate::{InfoHash, SingleTarget, Torrent};

    use super::TorrentList;

    fn dummy_list() -> TorrentList {
        TorrentList::from_vec(vec![
            Torrent::dummy_from_hash(
                &InfoHash::new("C811B41641A09D192B8ED81B14064FFF55D85CE3").unwrap(),
            ),
            Torrent::dummy_from_hash(
                &InfoHash::new("631a31dd0a46257d5078c0dee4e66e26f73e42ac")
                    .unwrap()
                    .hybrid(
                        &InfoHash::new(
                            "d8dd32ac93357c368556af3ac1d95c9d76bd0dff6fa9833ecdac3d53134efabb",
                        )
                        .unwrap(),
                    )
                    .unwrap(),
            ),
            Torrent::dummy_from_hash(
                &InfoHash::new("caf1e1c30e81cb361b9ee167c4aa64228a7fa4fa9f6105232b28ad099f3a302e")
                    .unwrap(),
            ),
        ])
    }

    #[test]
    fn matches_v1() {
        let list = dummy_list();
        let target = SingleTarget::new("C811B41641A09D192B8ED81B14064FFF55D85CE3").unwrap();

        let found = list.get(&target).unwrap();

        assert_eq!(
            found.hash,
            InfoHash::new("C811B41641A09D192B8ED81B14064FFF55D85CE3").unwrap()
        );
    }

    #[test]
    fn matches_hybrid_v2() {
        let list = dummy_list();
        let target =
            SingleTarget::new("d8dd32ac93357c368556af3ac1d95c9d76bd0dff6fa9833ecdac3d53134efabb")
                .unwrap();

        let found = list.get(&target).unwrap();

        assert_eq!(
            found.hash,
            InfoHash::new("631a31dd0a46257d5078c0dee4e66e26f73e42ac")
                .unwrap()
                .hybrid(
                    &InfoHash::new(
                        "d8dd32ac93357c368556af3ac1d95c9d76bd0dff6fa9833ecdac3d53134efabb"
                    )
                    .unwrap()
                )
                .unwrap()
        );
    }

    #[test]
    fn matches_hybrid_v2_truncated() {
        let list = dummy_list();
        let target = SingleTarget::new("d8dd32ac93357c368556af3ac1d95c9d76bd0dff").unwrap();

        let found = list.get(&target).unwrap();

        assert_eq!(
            found.hash,
            InfoHash::new("631a31dd0a46257d5078c0dee4e66e26f73e42ac")
                .unwrap()
                .hybrid(
                    &InfoHash::new(
                        "d8dd32ac93357c368556af3ac1d95c9d76bd0dff6fa9833ecdac3d53134efabb"
                    )
                    .unwrap()
                )
                .unwrap()
        );
    }

    #[test]
    fn matches_hybrid_v1() {
        let list = dummy_list();
        let target = SingleTarget::new("631a31dd0a46257d5078c0dee4e66e26f73e42ac").unwrap();

        let found = list.get(&target).unwrap();

        assert_eq!(
            found.hash,
            InfoHash::new("631a31dd0a46257d5078c0dee4e66e26f73e42ac")
                .unwrap()
                .hybrid(
                    &InfoHash::new(
                        "d8dd32ac93357c368556af3ac1d95c9d76bd0dff6fa9833ecdac3d53134efabb"
                    )
                    .unwrap()
                )
                .unwrap()
        );
    }

    #[test]
    fn matches_v2() {
        let list = dummy_list();
        let target =
            SingleTarget::new("caf1e1c30e81cb361b9ee167c4aa64228a7fa4fa9f6105232b28ad099f3a302e")
                .unwrap();

        let found = list.get(&target).unwrap();

        assert_eq!(
            found.hash,
            InfoHash::new("caf1e1c30e81cb361b9ee167c4aa64228a7fa4fa9f6105232b28ad099f3a302e")
                .unwrap()
        );
    }

    #[test]
    fn matches_v2_truncated() {
        let list = dummy_list();
        let target = SingleTarget::new("caf1e1c30e81cb361b9ee167c4aa64228a7fa4fa").unwrap();

        let found = list.get(&target).unwrap();

        assert_eq!(
            found.hash,
            InfoHash::new("caf1e1c30e81cb361b9ee167c4aa64228a7fa4fa9f6105232b28ad099f3a302e")
                .unwrap()
        );
    }
}
