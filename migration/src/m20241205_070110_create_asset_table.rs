use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20241204_122105_create_client_and_package_tables::Client, utils::default_uuid};

const ASSET_CLIENT_FK: &str = "fk-asset-client";
const ASSET_UUID_INDEX: &str = "idx-asset-uuid";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Asset::Table)
                    .if_not_exists()
                    .col(pk_auto(Asset::Id))
                    .col(
                        uuid(Asset::Uuid)
                            .unique_key()
                            .default(Value::Uuid(default_uuid())),
                    )
                    .col(string(Asset::Name).not_null())
                    .col(string(Asset::Description))
                    .col(string(Asset::IpfsHash).unique_key())
                    .col(string(Asset::NftId).unique_key())
                    .col(big_integer(Asset::ClientId))
                    .col(date_time(Asset::DateAdded).default(Expr::current_timestamp()))
                    .col(date_time(Asset::LastUpdated).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name(ASSET_CLIENT_FK)
                            .from(Asset::Table, Asset::ClientId)
                            .to(Client::Table, Client::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(ASSET_UUID_INDEX)
                    .table(Asset::Table)
                    .col(Asset::Uuid)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name(ASSET_UUID_INDEX)
                    .table(Asset::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Asset::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Asset {
    Table,
    Id,
    Uuid,
    Name,
    Description,
    IpfsHash,
    NftId,
    ClientId,
    DateAdded,
    LastUpdated,
}
