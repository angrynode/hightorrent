use std::str::FromStr;

use crate::{InfoHash, InfoHashError, TorrentID};

/// A single Torrent to interact with.
///
/// The torrent is matched ambiguously with the provided string, because when it is 40 characters
/// long, it could be either a full infohash v1, or a v2 [`TorrentID`](crate::id::TorrentID).
/// If the SingleTarget could match both, the truncated infohash v2 is always prefered, because a
/// truncated SHA256 hash is more resilient to collision attacks.
///
/// This type is useful so you do not confuse in your codebase actual
/// [`InfoHash`](crate::hash::InfoHash) with
/// [`TorrentID`](crate::id::TorrentID). A SingleTarget may be either.
///
/// There is no provided method to convert to a [`TorrentID`](crate::id::TorrentID) because
/// that would allow for logic errors (experienced first-hand). However, the
/// [`truncated`](crate::target::SingleTarget::truncated) method returns a string
/// truncated to 40 characters.
#[derive(Clone, Debug, PartialEq)]
pub struct SingleTarget(String);

impl SingleTarget {
    /// Create a new SingleTarget from a string. Will fail if the passed string cannot
    /// be parsed as an InfoHash (wrong characters / length).
    pub fn new(hash: &str) -> Result<SingleTarget, InfoHashError> {
        // Check that the passed string looks like a infohash
        let hash = InfoHash::new(hash)?;
        // Use the produced normalized (lowercase) hash
        Ok(SingleTarget(hash.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns a stringy representation of the SingleTarget, truncated to 40 characters
    /// This may or may not be an actual [`TorrentID`](crate::id::TorrentID) because
    /// the truncated SingleTarget, when it matches a hybrid's torrent infohash v1,
    /// is not the corresponding TorrentID, which would be the truncated infohash v2
    /// of said hybrid torrent.
    pub fn truncated(&self) -> &str {
        self.as_str().get(0..40).unwrap()
    }
}

impl std::fmt::Display for SingleTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SingleTarget {
    type Err = InfoHashError;

    fn from_str(value: &str) -> Result<SingleTarget, InfoHashError> {
        SingleTarget::new(value)
    }
}

/// Try to turn a stringy value into a [`SingleTarget`]. Fails if the value doesn't **look** like a
/// [`InfoHash`](crate::hash::InfoHash).
pub trait ToSingleTarget {
    fn to_single_target(&self) -> Result<SingleTarget, InfoHashError>;
}

impl ToSingleTarget for &str {
    fn to_single_target(&self) -> Result<SingleTarget, InfoHashError> {
        SingleTarget::new(self)
    }
}

impl ToSingleTarget for SingleTarget {
    fn to_single_target(&self) -> Result<SingleTarget, InfoHashError> {
        Ok(self.clone())
    }
}

impl From<InfoHash> for SingleTarget {
    fn from(value: InfoHash) -> SingleTarget {
        SingleTarget::new(value.as_str()).unwrap()
    }
}

impl From<&InfoHash> for SingleTarget {
    fn from(value: &InfoHash) -> SingleTarget {
        SingleTarget::new(value.as_str()).unwrap()
    }
}

impl From<TorrentID> for SingleTarget {
    fn from(value: TorrentID) -> SingleTarget {
        SingleTarget::new(value.as_str()).unwrap()
    }
}

impl From<&TorrentID> for SingleTarget {
    fn from(value: &TorrentID) -> SingleTarget {
        SingleTarget::new(value.as_str()).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Criteria to filter a [`TorrentList`](crate::list::TorrentList), returning multiple entries.
///
/// The following criteria are available:
///    - MultiTarget::All applies no filter
///    - MultiTarget::Hash filters a single torrent matching a given SingleTarget
///    - TODO: MultiTarget::Name
///    - TODO: MultiTarget::Tracker
///    - TODO: AND/OR/XOR for multiple criteria
pub enum MultiTarget {
    All,
    Hash(SingleTarget),
}

impl FromStr for MultiTarget {
    type Err = InfoHashError;

    #[allow(dead_code)]
    fn from_str(value: &str) -> Result<MultiTarget, Self::Err> {
        if value == "all" {
            Ok(MultiTarget::All)
        } else {
            Ok(MultiTarget::Hash(SingleTarget::new(value)?))
        }
    }
}

impl TryFrom<&str> for MultiTarget {
    type Error = InfoHashError;

    fn try_from(value: &str) -> Result<MultiTarget, Self::Error> {
        MultiTarget::from_str(&value)
    }
}

// Turn an InfoHash into a SingleTarget
impl From<InfoHash> for MultiTarget {
    fn from(h: InfoHash) -> MultiTarget {
        MultiTarget::Hash(SingleTarget::new(h.as_str()).unwrap())
    }
}

impl From<SingleTarget> for MultiTarget {
    fn from(value: SingleTarget) -> MultiTarget {
        MultiTarget::Hash(value)
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn singletarget_can_be_truncated() {
        let target =
            SingleTarget::new("abcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef1234")
                .unwrap();
        let truncated = target.truncated();
        assert_eq!(truncated.len(), 40);
        assert_eq!(truncated, "abcdefabcdefabcdefabcdefabcdefabcdefabcd");
    }

    #[test]
    fn singletarget_ignores_casing() {
        assert_eq!(
            SingleTarget::new("ABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEFABCDEF1234")
                .unwrap(),
            SingleTarget::new("abcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef1234")
                .unwrap()
        );
    }
}
