//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "subscription_package")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub uuid: Uuid,
    pub name: String,
    #[sea_orm(column_type = "Float")]
    pub price: f32,
    pub storage_capacity_mb: i64,
    pub monthly_requests: i64,
    pub max_allowed_sessions: i32,
    pub date_added: DateTime,
    pub last_updated: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::client_package_subscription::Entity")]
    ClientPackageSubscription,
}

impl Related<super::client_package_subscription::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ClientPackageSubscription.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}