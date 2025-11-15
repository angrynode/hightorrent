use hightorrent::MagnetLink;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "magnet")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub torrent_id: String,
    pub magnet: MagnetLink,
    pub name: String,
    pub resolved: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}
