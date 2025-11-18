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

pub mod mixed {
    use super::*;
    use sea_orm_migration::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "mixed")]
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
        pub mod m20251115_01_mixed {
            use sea_orm_migration::{prelude::*, schema::*};

            #[derive(DeriveMigrationName)]
            pub struct Migration;

            #[async_trait::async_trait]
            impl MigrationTrait for Migration {
                async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                    manager
                        .create_table(
                            Table::create()
                                .table(Mixed::Table)
                                .if_not_exists()
                                .col(pk_auto(Mixed::Id))
                                .col(string(Mixed::TorrentID).unique_key())
                                .col(string(Mixed::Magnet).unique_key())
                                .to_owned(),
                        )
                        .await
                }

                async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                    manager
                        .drop_table(Table::drop().table(Mixed::Table).to_owned())
                        .await
                }
            }

            #[derive(DeriveIden)]
            enum Mixed {
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
            vec![Box::new(migration::m20251115_01_mixed::Migration)]
        }
    }
}

pub mod optional_mixed {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "optional_mixed")]
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

        pub mod m20251118_01_optional_mixed {
            use sea_orm_migration::{prelude::*, schema::*};

            #[derive(DeriveMigrationName)]
            pub struct Migration;

            #[async_trait::async_trait]
            impl MigrationTrait for Migration {
                async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                    manager
                        .create_table(
                            Table::create()
                                .table(OptionalMixed::Table)
                                .if_not_exists()
                                .col(pk_auto(OptionalMixed::Id))
                                .col(string(OptionalMixed::TorrentID))
                                .col(string(OptionalMixed::Magnet))
                                .to_owned(),
                        )
                        .await
                }

                async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                    manager
                        .drop_table(Table::drop().table(OptionalMixed::Table).to_owned())
                        .await
                }
            }

            #[derive(DeriveIden)]
            enum OptionalMixed {
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
                vec![Box::new(m20251118_01_optional_mixed::Migration)]
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
async fn test_torrent_real_db() {
    use sea_orm_migration::*;

    let tmpdir = async_tempfile::TempDir::new().await.unwrap();
    let sqlite = tmpdir.join("mixed.sqlite");
    let sqlite_str = sqlite.to_str().unwrap();

    let db = sea_orm::Database::connect(&format!("sqlite://{}?mode=rwc", sqlite_str))
        .await
        .unwrap();
    mixed::Migrator::up(&db, None).await.unwrap();

    let magnet =
        MagnetLink::new(&std::fs::read_to_string("tests/bittorrent-v2-test.magnet").unwrap())
            .unwrap();

    let model = mixed::ActiveModel {
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

    let model2 = mixed::ActiveModel {
        torrent_id: Set(magnet2.id()),
        magnet: Set(magnet2.clone()),
        ..Default::default()
    }
    .save(&db)
    .await
    .unwrap();

    let nonactive_model = model.try_into_model().unwrap();
    let saved_model_by_id = mixed::Entity::find_by_id(nonactive_model.id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_model_by_id, nonactive_model);
    assert_eq!(saved_model_by_id.magnet, magnet);
    assert_eq!(nonactive_model.magnet, magnet);

    let nonactive_model2 = model2.try_into_model().unwrap();
    let saved_model_by_id2 = mixed::Entity::find_by_id(nonactive_model2.id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_model_by_id2, nonactive_model2);
    assert_eq!(saved_model_by_id2.magnet, magnet2);
    assert_eq!(nonactive_model2.magnet, magnet2);

    // Try query by TorrentID
    let saved_model_by_torrentid = mixed::Entity::find()
        .filter(mixed::Column::TorrentId.eq(magnet.id()))
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_model_by_id, nonactive_model);
    assert_eq!(saved_model_by_torrentid.magnet, magnet);

    // Try query by MagnetLink
    let saved_model_by_magnet = mixed::Entity::find()
        .filter(mixed::Column::Magnet.eq(magnet.clone()))
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_model_by_id, nonactive_model);
    assert_eq!(saved_model_by_magnet.magnet, magnet);

    // Try listing torrents
    let mut found_one = false;
    let mut found_two = false;
    let list = mixed::Entity::find().all(&db).await.unwrap();
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
    optional_mixed::migration::Migrator::up(&db, None)
        .await
        .unwrap();

    // Try with None
    let model = optional_mixed::ActiveModel {
        torrent_id: Set(None),
        magnet: Set(None),
        ..Default::default()
    }
    .save(&db)
    .await
    .unwrap();

    let nonactive_model = model.try_into_model().unwrap();
    let saved_model = optional_mixed::Entity::find()
        .filter(optional_mixed::Column::Magnet.eq(Option::<MagnetLink>::None))
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
    let sqlite = tmpdir.join("optional_mixed_none.sqlite");
    let sqlite_str = sqlite.to_str().unwrap();

    let db = sea_orm::Database::connect(&format!("sqlite://{}?mode=rwc", sqlite_str))
        .await
        .unwrap();
    optional_mixed::migration::Migrator::up(&db, None)
        .await
        .unwrap();

    // Try with None
    let model = optional_mixed::ActiveModel {
        torrent_id: NotSet,
        magnet: NotSet,
        ..Default::default()
    }
    .save(&db)
    .await
    .unwrap();

    let nonactive_model = model.try_into_model().unwrap();
    let saved_model = optional_mixed::Entity::find()
        .filter(optional_mixed::Column::Magnet.eq(Option::<MagnetLink>::None))
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved_model.magnet.as_ref(), None);
    assert_eq!(nonactive_model.magnet.as_ref(), None);
}
