//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "asset")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    #[sea_orm(column_type = "Double")]
    pub size_mb: f64,
    #[sea_orm(unique)]
    pub ipfs_hash: String,
    #[sea_orm(unique)]
    pub nft_id: i64,
    pub client_id: i64,
    pub folder_id: i64,
    pub date_added: DateTime,
    pub last_updated: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::client::Entity",
        from = "Column::ClientId",
        to = "super::client::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Client2,
    #[sea_orm(
        belongs_to = "super::client::Entity",
        from = "(Column::FolderId, Column::FolderId, Column::FolderId, Column::FolderId)",
        to = "(super::client::Column::Id, super::client::Column::Id, super::client::Column::Id, super::client::Column::Id)",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Client1,
}

impl ActiveModelBehavior for ActiveModel {}
