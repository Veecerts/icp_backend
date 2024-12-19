use sea_orm_migration::{prelude::*, schema::*};

use crate::utils::default_uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

const USER_UUID_INDEX: &str = "idx-user-uuid";
const PROFILE_UUID_INDEX: &str = "idx-profile-uuid";
const PROFILE_USER_FOREIGN_KEY: &str = "fk-profile-user";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(
                        uuid(User::Uuid)
                            .not_null()
                            .unique_key()
                            .default(Value::Uuid(default_uuid())),
                    )
                    .col(string(User::Email).not_null().unique_key())
                    .col(ColumnDef::new(User::WalletAddress).string().null())
                    .col(ColumnDef::new(User::PasswordHash).string().null())
                    .col(
                        date_time(User::DateAdded)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        date_time(User::LastUpdated)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Profile::Table)
                    .if_not_exists()
                    .col(pk_auto(Profile::Id))
                    .col(
                        uuid(Profile::Uuid)
                            .not_null()
                            .unique_key()
                            .default(Value::Uuid(default_uuid())),
                    )
                    .col(ColumnDef::new(Profile::FirstName).string().null())
                    .col(ColumnDef::new(Profile::LastName).string().null())
                    .col(ColumnDef::new(Profile::ImageHash).string().null())
                    .col(big_integer(Profile::UserId).not_null().unique_key())
                    .col(
                        date_time(Profile::DateAdded)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        date_time(Profile::LastUpdated)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(PROFILE_USER_FOREIGN_KEY)
                            .from(Profile::Table, Profile::UserId)
                            .to(User::Table, User::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name(USER_UUID_INDEX)
                    .table(User::Table)
                    .col(User::Uuid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name(PROFILE_UUID_INDEX)
                    .table(Profile::Table)
                    .col(Profile::Uuid)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name(PROFILE_USER_FOREIGN_KEY)
                    .table(Profile::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Profile::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name(USER_UUID_INDEX)
                    .table(User::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name(PROFILE_UUID_INDEX)
                    .table(Profile::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Uuid,
    Email,
    WalletAddress,
    PasswordHash,
    DateAdded,
    LastUpdated,
}

#[derive(DeriveIden)]
pub enum Profile {
    Table,
    Id,
    Uuid,
    FirstName,
    LastName,
    ImageHash,
    UserId,
    DateAdded,
    LastUpdated,
}
