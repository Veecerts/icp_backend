use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20241204_111833_create_user_tables::User, utils::default_uuid};

#[derive(DeriveMigrationName)]
pub struct Migration;

const CLIENT_ACTIVE_SUBSCRIPTION_FK: &str = "fk-client-subscription-package";
const CLIENT_USER_FK: &str = "fk-client-user";

const CLIENT_UUID_INDEX: &str = "idx-client-uuid";
const SUBSCRIPTION_PACKAGE_UUID_INDEX: &str = "idx-subscription-package-uuid";

const CLIENT_PACKAGE_SUBSCRIPTION_CLIENT_FK: &str = "fk-client-package-subscription-client";
const CLIENT_PACKAGE_SUBSCRIPTION_PACKAGE_FK: &str = "fk-client-package-subscription-package";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Client::Table)
                    .if_not_exists()
                    .col(pk_auto(Client::Id))
                    .col(
                        uuid(Client::Uuid)
                            .not_null()
                            .unique_key()
                            .default(Value::Uuid(default_uuid())),
                    )
                    .col(big_integer(Client::UserId).not_null().unique_key())
                    .col(big_integer(Client::ActiveSubscriptionId))
                    .col(string(Client::ApiSecretHash).not_null().unique_key())
                    .col(
                        date_time(Client::DateAdded)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        date_time(Client::LastUpdated)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(CLIENT_USER_FK)
                            .from(Client::Table, Client::UserId)
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
                    .table(SubscriptionPackage::Table)
                    .if_not_exists()
                    .col(pk_auto(SubscriptionPackage::Id))
                    .col(
                        uuid(SubscriptionPackage::Uuid)
                            .not_null()
                            .unique_key()
                            .default(Value::Uuid(default_uuid())),
                    )
                    .col(string(SubscriptionPackage::Name).not_null())
                    .col(float(SubscriptionPackage::Price).not_null())
                    .col(big_integer(SubscriptionPackage::StorageCapacityMb).not_null())
                    .col(big_integer(SubscriptionPackage::MonthlyRequests).not_null())
                    .col(integer(SubscriptionPackage::MaxAllowedSessions).not_null())
                    .col(
                        date_time(SubscriptionPackage::DateAdded)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        date_time(SubscriptionPackage::LastUpdated)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ClientPackageSubscription::Table)
                    .if_not_exists()
                    .col(pk_auto(ClientPackageSubscription::Id))
                    .col(
                        uuid(ClientPackageSubscription::Uuid)
                            .not_null()
                            .unique_key()
                            .default(Value::Uuid(default_uuid())),
                    )
                    .col(big_integer(ClientPackageSubscription::ClientId).not_null())
                    .col(big_integer(ClientPackageSubscription::SubscriptionPackageId).not_null())
                    .col(float(ClientPackageSubscription::Amount).not_null())
                    .col(
                        date_time(ClientPackageSubscription::DateAdded)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(date_time(ClientPackageSubscription::ExpiresAt).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name(CLIENT_PACKAGE_SUBSCRIPTION_CLIENT_FK)
                            .from(
                                ClientPackageSubscription::Table,
                                ClientPackageSubscription::ClientId,
                            )
                            .to(Client::Table, Client::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(CLIENT_PACKAGE_SUBSCRIPTION_PACKAGE_FK)
                            .from(
                                ClientPackageSubscription::Table,
                                ClientPackageSubscription::SubscriptionPackageId,
                            )
                            .to(SubscriptionPackage::Table, SubscriptionPackage::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name(CLIENT_ACTIVE_SUBSCRIPTION_FK)
                    .from(Client::Table, Client::ActiveSubscriptionId)
                    .to(
                        ClientPackageSubscription::Table,
                        ClientPackageSubscription::Id,
                    )
                    .on_update(ForeignKeyAction::Cascade)
                    .on_delete(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(CLIENT_UUID_INDEX)
                    .if_not_exists()
                    .table(Client::Table)
                    .col(Client::Uuid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(SUBSCRIPTION_PACKAGE_UUID_INDEX)
                    .if_not_exists()
                    .table(SubscriptionPackage::Table)
                    .col(SubscriptionPackage::Uuid)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name(CLIENT_USER_FK)
                    .table(Client::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name(CLIENT_ACTIVE_SUBSCRIPTION_FK)
                    .table(Client::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name(CLIENT_UUID_INDEX)
                    .if_exists()
                    .table(Client::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name(SUBSCRIPTION_PACKAGE_UUID_INDEX)
                    .if_exists()
                    .table(SubscriptionPackage::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Client::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(SubscriptionPackage::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(ClientPackageSubscription::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Client {
    Table,
    Id,
    Uuid,
    UserId,
    ActiveSubscriptionId,
    ApiSecretHash,
    DateAdded,
    LastUpdated,
}

#[derive(DeriveIden)]
pub enum SubscriptionPackage {
    Table,
    Id,
    Uuid,
    Name,
    Price,
    StorageCapacityMb,
    MonthlyRequests,
    MaxAllowedSessions,
    DateAdded,
    LastUpdated,
}

#[derive(DeriveIden)]
pub enum ClientPackageSubscription {
    Table,
    Id,
    Uuid,
    ClientId,
    SubscriptionPackageId,
    Amount,
    DateAdded,
    ExpiresAt,
}
