use hightorrent::{MagnetLink, TorrentFile, TorrentID};
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

pub mod mixed_magnet {
    use super::*;
    use sea_orm_migration::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "mixed_magnet")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        #[sea_orm(unique)]
        pub torrent_id: TorrentID,
        #[sea_orm(unique)]
        pub magnet: MagnetLink,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    #[async_trait::async_trait]
    impl ActiveModelBehavior for ActiveModel {}

    pub mod migration {
        pub mod m20251115_01_mixed_magnet {
            use sea_orm_migration::{prelude::*, schema::*};

            #[derive(DeriveMigrationName)]
            pub struct Migration;

            #[async_trait::async_trait]
            impl MigrationTrait for Migration {
                async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                    manager
                        .create_table(
                            Table::create()
                                .table(MixedMagnet::Table)
                                .if_not_exists()
                                .col(pk_auto(MixedMagnet::Id))
                                .col(string(MixedMagnet::TorrentID).unique_key())
                                .col(string(MixedMagnet::Magnet).unique_key())
                                .to_owned(),
                        )
                        .await
                }

                async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                    manager
                        .drop_table(Table::drop().table(MixedMagnet::Table).to_owned())
                        .await
                }
            }

            #[derive(DeriveIden)]
            enum MixedMagnet {
                Table,
                Id,
                TorrentID,
                Magnet,
            }
        }
    }

    pub struct Migrator;

    #[async_trait::async_trait]
    impl MigratorTrait for Migrator {
        fn migrations() -> Vec<Box<dyn MigrationTrait>> {
            vec![Box::new(migration::m20251115_01_mixed_magnet::Migration)]
        }
    }
}

pub mod mixed_torrent {
    use super::*;
    use sea_orm_migration::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "mixed_torrent")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        #[sea_orm(unique)]
        pub torrent_id: TorrentID,
        #[sea_orm(unique)]
        pub torrent_file: TorrentFile,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    #[async_trait::async_trait]
    impl ActiveModelBehavior for ActiveModel {}

    pub mod migration {
        pub mod m20251115_01_mixed_torrent {
            use sea_orm_migration::{prelude::*, schema::*};

            #[derive(DeriveMigrationName)]
            pub struct Migration;

            #[async_trait::async_trait]
            impl MigrationTrait for Migration {
                async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                    manager
                        .create_table(
                            Table::create()
                                .table(MixedTorrent::Table)
                                .if_not_exists()
                                .col(pk_auto(MixedTorrent::Id))
                                .col(string(MixedTorrent::TorrentID).unique_key())
                                .col(var_binary(MixedTorrent::TorrentFile, 0).unique_key())
                                .to_owned(),
                        )
                        .await
                }

                async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                    manager
                        .drop_table(Table::drop().table(MixedTorrent::Table).to_owned())
                        .await
                }
            }

            #[derive(DeriveIden)]
            enum MixedTorrent {
                Table,
                Id,
                TorrentID,
                TorrentFile,
            }
        }
    }

    pub struct Migrator;

    #[async_trait::async_trait]
    impl MigratorTrait for Migrator {
        fn migrations() -> Vec<Box<dyn MigrationTrait>> {
            vec![Box::new(migration::m20251115_01_mixed_torrent::Migration)]
        }
    }
}

pub mod optional_mixed_magnet {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "optional_mixed_magnet")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub torrent_id: Option<TorrentID>,
        pub magnet: Option<MagnetLink>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    #[async_trait::async_trait]
    impl ActiveModelBehavior for ActiveModel {}

    pub mod migration {
        use sea_orm_migration::prelude::*;

        pub mod m20251118_01_optional_mixed_magnet {
            use sea_orm_migration::{prelude::*, schema::*};

            #[derive(DeriveMigrationName)]
            pub struct Migration;

            #[async_trait::async_trait]
            impl MigrationTrait for Migration {
                async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                    manager
                        .create_table(
                            Table::create()
                                .table(OptionalMixedMagnet::Table)
                                .if_not_exists()
                                .col(pk_auto(OptionalMixedMagnet::Id))
                                .col(string_null(OptionalMixedMagnet::TorrentID))
                                .col(string_null(OptionalMixedMagnet::Magnet))
                                .to_owned(),
                        )
                        .await
                }

                async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                    manager
                        .drop_table(Table::drop().table(OptionalMixedMagnet::Table).to_owned())
                        .await
                }
            }

            #[derive(DeriveIden)]
            enum OptionalMixedMagnet {
                Table,
                Id,
                TorrentID,
                Magnet,
            }
        }

        pub struct Migrator;

        #[async_trait::async_trait]
        impl MigratorTrait for Migrator {
            fn migrations() -> Vec<Box<dyn MigrationTrait>> {
                vec![Box::new(m20251118_01_optional_mixed_magnet::Migration)]
            }
        }
    }
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

#[tokio::test]
async fn test_magnet_real_db() {
    use sea_orm_migration::*;

    let tmpdir = async_tempfile::TempDir::new().await.unwrap();
    let sqlite = tmpdir.join("mixed_magnet.sqlite");
    let sqlite_str = sqlite.to_str().unwrap();

    let db = sea_orm::Database::connect(&format!("sqlite://{}?mode=rwc", sqlite_str))
        .await
        .unwrap();
    mixed_magnet::Migrator::up(&db, None).await.unwrap();

    let magnet =
        MagnetLink::new(&std::fs::read_to_string("tests/bittorrent-v2-test.magnet").unwrap())
            .unwrap();

    let model = mixed_magnet::ActiveModel {
        torrent_id: Set(magnet.id()),
        magnet: Set(magnet.clone()),
        ..Default::default()
    }
    .save(&db)
    .await
    .unwrap();

    let magnet2 = MagnetLink::new(
        &std::fs::read_to_string("tests/bittorrent-v2-hybrid-test.magnet").unwrap(),
    )
    .unwrap();

    let model2 = mixed_magnet::ActiveModel {
        torrent_id: Set(magnet2.id()),
        magnet: Set(magnet2.clone()),
        ..Default::default()
    }
    .save(&db)
    .await
    .unwrap();

    let nonactive_model = model.try_into_model().unwrap();
    let saved_model_by_id = mixed_magnet::Entity::find_by_id(nonactive_model.id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_model_by_id, nonactive_model);
    assert_eq!(saved_model_by_id.magnet, magnet);
    assert_eq!(nonactive_model.magnet, magnet);

    let nonactive_model2 = model2.try_into_model().unwrap();
    let saved_model_by_id2 = mixed_magnet::Entity::find_by_id(nonactive_model2.id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_model_by_id2, nonactive_model2);
    assert_eq!(saved_model_by_id2.magnet, magnet2);
    assert_eq!(nonactive_model2.magnet, magnet2);

    // Try query by TorrentID
    let saved_model_by_torrentid = mixed_magnet::Entity::find()
        .filter(mixed_magnet::Column::TorrentId.eq(magnet.id()))
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_model_by_id, nonactive_model);
    assert_eq!(saved_model_by_torrentid.magnet, magnet);

    // Try query by MagnetLink
    let saved_model_by_magnet = mixed_magnet::Entity::find()
        .filter(mixed_magnet::Column::Magnet.eq(magnet.clone()))
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_model_by_id, nonactive_model);
    assert_eq!(saved_model_by_magnet.magnet, magnet);

    // Try listing torrents
    let mut found_one = false;
    let mut found_two = false;
    let list = mixed_magnet::Entity::find().all(&db).await.unwrap();
    println!("{:?}", list);
    for entry in list {
        if entry.magnet == magnet && entry.torrent_id == magnet.id() {
            found_one = true;
            continue;
        }

        if entry.magnet == magnet2 && entry.torrent_id == magnet2.id() {
            found_two = true;
        }
    }

    assert!(found_one);
    assert!(found_two);
}

#[tokio::test]
async fn test_torrent_real_optional_none() {
    use sea_orm_migration::*;

    let tmpdir = async_tempfile::TempDir::new().await.unwrap();
    let sqlite = tmpdir.join("optional_mixed_none.sqlite");
    let sqlite_str = sqlite.to_str().unwrap();

    let db = sea_orm::Database::connect(&format!("sqlite://{}?mode=rwc", sqlite_str))
        .await
        .unwrap();
    optional_mixed_magnet::migration::Migrator::up(&db, None)
        .await
        .unwrap();

    // Try with None
    let model = optional_mixed_magnet::ActiveModel {
        torrent_id: Set(None),
        magnet: Set(None),
        ..Default::default()
    }
    .save(&db)
    .await
    .unwrap();

    let list = optional_mixed_magnet::Entity::find()
        .all(&db)
        .await
        .unwrap();
    for entry in list {
        println!("- {:?}", entry);
    }

    let nonactive_model = model.try_into_model().unwrap();
    let saved_model = optional_mixed_magnet::Entity::find()
        .filter(optional_mixed_magnet::Column::Magnet.is_null())
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_model.magnet.as_ref(), None);
    assert_eq!(nonactive_model.magnet.as_ref(), None);
}

#[tokio::test]
async fn test_torrent_real_optional_notset() {
    use sea_orm_migration::*;

    let tmpdir = async_tempfile::TempDir::new().await.unwrap();
    let sqlite = tmpdir.join("optional_mixed_magnet_none.sqlite");
    let sqlite_str = sqlite.to_str().unwrap();

    let db = sea_orm::Database::connect(&format!("sqlite://{}?mode=rwc", sqlite_str))
        .await
        .unwrap();
    optional_mixed_magnet::migration::Migrator::up(&db, None)
        .await
        .unwrap();

    // Try with None
    let model = optional_mixed_magnet::ActiveModel {
        torrent_id: NotSet,
        magnet: NotSet,
        ..Default::default()
    }
    .save(&db)
    .await
    .unwrap();

    let list = optional_mixed_magnet::Entity::find()
        .all(&db)
        .await
        .unwrap();
    for entry in list {
        println!("- {:?}", entry);
    }

    let nonactive_model = model.try_into_model().unwrap();
    let saved_model = optional_mixed_magnet::Entity::find()
        .filter(optional_mixed_magnet::Column::Magnet.is_null())
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_model.magnet.as_ref(), None);
    assert_eq!(nonactive_model.magnet.as_ref(), None);
}

#[tokio::test]
async fn test_torrent_real_db() {
    use sea_orm_migration::*;

    let tmpdir = async_tempfile::TempDir::new().await.unwrap();
    let sqlite = tmpdir.join("mixed.sqlite");
    let sqlite_str = sqlite.to_str().unwrap();

    let db = sea_orm::Database::connect(&format!("sqlite://{}?mode=rwc", sqlite_str))
        .await
        .unwrap();
    mixed_torrent::Migrator::up(&db, None).await.unwrap();

    let torrent =
        TorrentFile::from_slice(&std::fs::read("tests/bittorrent-v2-test.torrent").unwrap())
            .unwrap();

    let model = mixed_torrent::ActiveModel {
        torrent_id: Set(torrent.id()),
        torrent_file: Set(torrent.clone()),
        ..Default::default()
    }
    .save(&db)
    .await
    .unwrap();

    let torrent2 =
        TorrentFile::from_slice(&std::fs::read("tests/bittorrent-v2-hybrid-test.torrent").unwrap())
            .unwrap();

    let model2 = mixed_torrent::ActiveModel {
        torrent_id: Set(torrent2.id()),
        torrent_file: Set(torrent2.clone()),
        ..Default::default()
    }
    .save(&db)
    .await
    .unwrap();

    let nonactive_model = model.try_into_model().unwrap();
    let saved_model_by_id = mixed_torrent::Entity::find_by_id(nonactive_model.id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_model_by_id, nonactive_model);
    assert_eq!(saved_model_by_id.torrent_file, torrent);
    assert_eq!(nonactive_model.torrent_file, torrent);

    let nonactive_model2 = model2.try_into_model().unwrap();
    let saved_model_by_id2 = mixed_torrent::Entity::find_by_id(nonactive_model2.id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_model_by_id2, nonactive_model2);
    assert_eq!(saved_model_by_id2.torrent_file, torrent2);
    assert_eq!(nonactive_model2.torrent_file, torrent2);

    // Try query by TorrentID
    let saved_model_by_torrentid = mixed_torrent::Entity::find()
        .filter(mixed_torrent::Column::TorrentId.eq(torrent.id()))
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_model_by_id, nonactive_model);
    assert_eq!(saved_model_by_torrentid.torrent_file, torrent);

    // Try query by TorrentFile
    let saved_model_by_torrent = mixed_torrent::Entity::find()
        .filter(mixed_torrent::Column::TorrentFile.eq(torrent.clone()))
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_model_by_id, nonactive_model);
    assert_eq!(saved_model_by_torrent.torrent_file, torrent);

    // Try listing torrents
    let mut found_one = false;
    let mut found_two = false;
    let list = mixed_torrent::Entity::find().all(&db).await.unwrap();
    println!("{:?}", list);
    for entry in list {
        if entry.torrent_file == torrent && entry.torrent_id == torrent.id() {
            found_one = true;
            continue;
        }

        if entry.torrent_file == torrent2 && entry.torrent_id == torrent2.id() {
            found_two = true;
        }
    }

    assert!(found_one);
    assert!(found_two);
}
