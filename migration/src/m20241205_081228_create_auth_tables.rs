use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20241204_111833_create_user_tables::User,
    m20241204_122105_create_client_and_package_tables::Client,
};

const AUTH_TOKEN_USER_FK: &str = "fk-auth-token-user";
const AUTH_TOKEN_UUID_INDEX: &str = "idx-auth-token-uuid";

const CLIENT_AUTH_TOKEN_USER_FK: &str = "fk-client-auth-token-user";
const CLIENT_AUTH_TOKEN_UUID_INDEX: &str = "idx-client-auth-token-uuid";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AuthToken::Table)
                    .if_not_exists()
                    .col(pk_auto(AuthToken::Id))
                    .col(uuid(AuthToken::Uuid).unique_key())
                    .col(string(AuthToken::Token))
                    .col(big_integer(AuthToken::UserId).unique_key())
                    .col(date_time(AuthToken::DateAdded).default(Expr::current_timestamp()))
                    .col(date_time(AuthToken::ExpiresAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name(AUTH_TOKEN_USER_FK)
                            .from(AuthToken::Table, AuthToken::UserId)
                            .to(User::Table, User::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ClientAuthToken::Table)
                    .if_not_exists()
                    .col(pk_auto(ClientAuthToken::Id))
                    .col(uuid(ClientAuthToken::Uuid).unique_key())
                    .col(string(ClientAuthToken::Token))
                    .col(big_integer(ClientAuthToken::ClientId))
                    .col(date_time(ClientAuthToken::DateAdded).default(Expr::current_timestamp()))
                    .col(date_time(ClientAuthToken::ExpiresAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name(CLIENT_AUTH_TOKEN_USER_FK)
                            .from(ClientAuthToken::Table, ClientAuthToken::ClientId)
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
                    .name(AUTH_TOKEN_UUID_INDEX)
                    .if_not_exists()
                    .table(AuthToken::Table)
                    .col(AuthToken::Uuid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(CLIENT_AUTH_TOKEN_UUID_INDEX)
                    .if_not_exists()
                    .table(ClientAuthToken::Table)
                    .col(ClientAuthToken::Uuid)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name(AUTH_TOKEN_UUID_INDEX)
                    .if_exists()
                    .table(AuthToken::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name(CLIENT_AUTH_TOKEN_UUID_INDEX)
                    .if_exists()
                    .table(ClientAuthToken::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(AuthToken::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(ClientAuthToken::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum AuthToken {
    Table,
    Id,
    Uuid,
    Token,
    UserId,
    DateAdded,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum ClientAuthToken {
    Table,
    Id,
    Uuid,
    Token,
    ClientId,
    DateAdded,
    ExpiresAt,
}
