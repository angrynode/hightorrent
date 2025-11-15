use hightorrent::{MagnetLink, TorrentID};
use sea_orm::entity::prelude::*;
use sea_orm::*;

mod id {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "magnet")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub torrent_id: TorrentID,
        pub magnet: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    #[async_trait::async_trait]
    impl ActiveModelBehavior for ActiveModel {}
}

mod magnet {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "magnet")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub torrent_id: String,
        pub magnet: MagnetLink,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    #[async_trait::async_trait]
    impl ActiveModelBehavior for ActiveModel {}
}

#[test]
fn test_magnet_active_model() {
    let magnet =
        MagnetLink::new(&std::fs::read_to_string("tests/bittorrent-v2-test.magnet").unwrap())
            .unwrap();

    let _model = magnet::ActiveModel {
        torrent_id: Set(magnet.id().to_string()),
        magnet: Set(magnet.clone()),
        ..Default::default()
    };
}

#[test]
fn test_torrentid_active_model() {
    let magnet =
        MagnetLink::new(&std::fs::read_to_string("tests/bittorrent-v2-test.magnet").unwrap())
            .unwrap();

    let _model = id::ActiveModel {
        torrent_id: Set(magnet.id()),
        magnet: Set(magnet.to_string()),
        ..Default::default()
    };
}
