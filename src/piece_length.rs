use serde::{Deserialize, Deserializer, Serialize};

use crate::TorrentFileError;

/// Maximum size of the `piece length` entry in info dict for V2 torrents.
///
/// Magic number copied over [from libtorrent](https://github.com/arvidn/libtorrent/blob/1b9dc7462f22bc1513464d01c72281280a6a5f97/include/libtorrent/file_storage.hpp#L246).
pub const PIECE_LENGTH_MAXIMUM: u32 = 536854528;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct PieceLength(pub u32);

impl PartialEq<PieceLength> for u32 {
    fn eq(&self, other: &PieceLength) -> bool {
        other.0 == *self
    }
}

impl<'de> Deserialize<'de> for PieceLength {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let len = u32::deserialize(deserializer)?;

        if len > PIECE_LENGTH_MAXIMUM || len == 0 {
            // return Err(serde::de::Error::custom(&format!("Invalid piece length: {len}")));
            return Err(serde::de::Error::custom(TorrentFileError::BadPieceLength {
                piece_length: len,
            }));
        }
        Ok(PieceLength(len))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bt_bencode::{
        value::{Number, Value},
        Error,
    };

    #[test]
    fn piece_length_correct() {
        let v = Value::Int(Number::Unsigned(100));
        let res: Result<PieceLength, Error> = bt_bencode::from_value(v);

        assert!(res.is_ok());
        assert_eq!(100, res.unwrap());
    }

    #[test]
    fn piece_length_negative() {
        let v = Value::Int(Number::Signed(-100));
        let res: Result<PieceLength, Error> = bt_bencode::from_value(v);

        assert!(res.is_err());
    }

    #[test]
    fn piece_length_zero() {
        let v = Value::Int(Number::Signed(0));
        let res: Result<PieceLength, Error> = bt_bencode::from_value(v);

        assert!(res.is_err());
    }

    #[test]
    fn piece_length_maximum() {
        let v = Value::Int(Number::Unsigned(PIECE_LENGTH_MAXIMUM.into()));
        let res: Result<PieceLength, Error> = bt_bencode::from_value(v);

        assert!(res.is_ok());
        assert_eq!(PIECE_LENGTH_MAXIMUM, res.unwrap());
    }

    #[test]
    fn piece_length_over_maximum() {
        let v = Value::Int(Number::Signed(u32::MAX.into()));
        let res: Result<PieceLength, Error> = bt_bencode::from_value(v);

        assert!(res.is_err());
    }
}
