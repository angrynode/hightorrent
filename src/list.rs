use crate::{InfoHash, SingleTarget, Torrent};

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
            .find(|t| {
                match &t.hash {
                    InfoHash::V1(h) | InfoHash::V2(h) => h.as_str() == target.as_str(),
                    InfoHash::Hybrid((v1, _v2)) => {
                        // Priority is given to matching v2, for more resilience to collision attacks
                        // but we can still match hybrid by infohash v1 SingleTarget
                        t.id.as_str() == target.truncated() || v1 == target.as_str()
                    }
                }
            })
            .map(|t| t.clone())
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
