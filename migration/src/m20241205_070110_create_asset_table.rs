use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20241204_122105_create_client_and_package_tables::Client, utils::default_uuid};

const ASSET_CLIENT_FK: &str = "fk-asset-client";
const ASSET_FOLDER_FK: &str = "fk-folder-client";
const ASSET_UUID_INDEX: &str = "idx-asset-uuid";

const FOLDER_CLIENT_FK: &str = "fk-folder-client";
const FOLDER_UUID_INDEX: &str = "idx-folder-uuid";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Folder::Table)
                    .if_not_exists()
                    .col(pk_auto(Folder::Id))
                    .col(uuid(Folder::Uuid).unique_key())
                    .col(string(Folder::Name))
                    .col(string(Folder::Description))
                    .col(big_integer(Folder::ClientId))
                    .col(date_time(Folder::DateAdded).default(Expr::current_timestamp()))
                    .col(date_time(Folder::LastUpdated).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name(FOLDER_CLIENT_FK)
                            .from(Folder::Table, Folder::ClientId)
                            .to(Client::Table, Client::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

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
                    .col(string(Asset::Name))
                    .col(string(Asset::Description))
                    .col(double(Asset::SizeMb))
                    .col(string(Asset::IpfsHash).unique_key())
                    .col(big_integer(Asset::NftId).unique_key())
                    .col(big_integer(Asset::ClientId))
                    .col(big_integer(Asset::FolderId))
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
                    .foreign_key(
                        ForeignKey::create()
                            .name(ASSET_FOLDER_FK)
                            .from(Asset::Table, Asset::FolderId)
                            .to(Folder::Table, Folder::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(FOLDER_UUID_INDEX)
                    .if_not_exists()
                    .table(Folder::Table)
                    .col(Folder::Uuid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(ASSET_UUID_INDEX)
                    .if_not_exists()
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
                    .name(FOLDER_UUID_INDEX)
                    .if_exists()
                    .table(Folder::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name(ASSET_UUID_INDEX)
                    .if_exists()
                    .table(Asset::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Asset::Table).if_exists().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Folder {
    Table,
    Id,
    Uuid,
    Name,
    Description,
    ClientId,
    DateAdded,
    LastUpdated,
}

#[derive(DeriveIden)]
pub enum Asset {
    Table,
    Id,
    Uuid,
    Name,
    Description,
    SizeMb,
    IpfsHash,
    NftId,
    ClientId,
    FolderId,
    DateAdded,
    LastUpdated,
}
