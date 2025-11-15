use serde::{Deserialize, Serialize};

use std::str::FromStr;

use crate::{InfoHash, InfoHashError};

/// An infohash string truncated to 40 characters.
///
/// This representation is used by libtorrent, among others, for interoperability with software
/// that was meant to be 40-characters v1 infohashes. For v1 infohashes, the string representation
/// of the TorrentID is identical. For hybrid and v2 infohashes however, it is truncated to 40
/// characters.
///
/// A TorrentID can be generated from a string, and if it is not valid will return an
/// [`InfoHashError`](crate::hash::InfoHashError). It can also be generated from an actual
/// [`InfoHash`](crate::hash::InfoHash) with the
/// [`TorrentID::from_infohash`](crate::id::TorrentID::from_infohash) and
/// [`InfoHash::id`](crate::hash::InfoHash::id) methods.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TorrentID(String);

impl TorrentID {
    pub fn new<T: AsRef<str>>(s: T) -> Result<TorrentID, InfoHashError> {
        Self::from_str(s.as_ref())
    }

    pub fn from_infohash(hash: &InfoHash) -> TorrentID {
        match hash {
            InfoHash::V1(v2hash) => TorrentID(v2hash.to_string()),
            InfoHash::V2(v2hash) | InfoHash::Hybrid((_, v2hash)) => {
                let mut truncated = v2hash.to_string();
                truncated.truncate(40);
                TorrentID(truncated)
            }
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TorrentID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for TorrentID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl FromStr for TorrentID {
    type Err = InfoHashError;

    fn from_str(s: &str) -> Result<TorrentID, InfoHashError> {
        let hash = InfoHash::new(s)?;
        Ok(Self::from_infohash(&hash))
    }
}

#[cfg(feature = "sea_orm")]
impl From<TorrentID> for sea_orm::sea_query::Value {
    fn from(id: TorrentID) -> Self {
        Self::String(Some(id.to_string()))
    }
}

#[cfg(feature = "sea_orm")]
impl sea_orm::TryGetable for TorrentID {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::error::TryGetError> {
        let val: String = res.try_get_by(index)?;
        TorrentID::new(&val).map_err(|e| {
            sea_orm::error::TryGetError::DbErr(sea_orm::DbErr::TryIntoErr {
                from: "String",
                into: "TorrentID",
                source: std::sync::Arc::new(e),
            })
        })
    }
}

#[cfg(feature = "sea_orm")]
impl sea_orm::sea_query::ValueType for TorrentID {
    fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        match v {
            // TODO: What to do with None String?
            // This should probably work with Option<MagnetLink> but not with MagnetLink
            // but i have no idea how sea orm works...
            sea_orm::Value::String(Some(s)) => {
                TorrentID::new(&s).map_err(|_e| sea_orm::sea_query::ValueTypeErr)
            }
            _ => Err(sea_orm::sea_query::ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "TorrentID".to_string()
    }

    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::String
    }

    fn column_type() -> sea_orm::sea_query::ColumnType {
        sea_orm::sea_query::ColumnType::String(sea_orm::sea_query::table::StringLen::N(40))
    }
}

#[cfg(feature = "sea_orm")]
impl sea_orm::sea_query::Nullable for TorrentID {
    fn null() -> sea_orm::sea_query::Value {
        sea_orm::sea_query::Value::String(None)
    }
}
