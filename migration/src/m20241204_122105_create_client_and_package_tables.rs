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

const CLIENT_USAGE_CLIENT_FK: &str = "fk-client-usage-client";
const CLIENT_USAGE_UUID_INDEX: &str = "idx-client-usage-uuid";

const CLIENT_MONTHLY_REQUESTS_CLIENT_FK: &str = "fk-client-monthly-requests-client";
const CLIENT_MONTHLY_REQUESTS_INDEX: &str = "idx-monthly-requests-uuid";

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
                            .unique_key()
                            .default(Value::Uuid(default_uuid())),
                    )
                    .col(big_integer(Client::UserId).unique_key())
                    .col(big_integer(Client::ActiveSubscriptionId))
                    .col(string(Client::ApiSecretHash).unique_key())
                    .col(date_time(Client::DateAdded).default(Expr::current_timestamp()))
                    .col(date_time(Client::LastUpdated).default(Expr::current_timestamp()))
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
                    .table(ClientUsage::Table)
                    .if_not_exists()
                    .col(pk_auto(ClientUsage::Id))
                    .col(uuid(ClientUsage::Uuid).unique_key())
                    .col(big_integer(ClientUsage::ClientId).unique_key())
                    .col(double(ClientUsage::UsedStorageMb).default(Value::BigInt(Some(0))))
                    .col(integer(ClientUsage::ActiveSessions).default(Value::Int(Some(0))))
                    .col(date_time(ClientUsage::DateAdded).default(Expr::current_timestamp()))
                    .col(date_time(ClientUsage::LastUpdated).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name(CLIENT_USAGE_CLIENT_FK)
                            .from(ClientUsage::Table, ClientUsage::ClientId)
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
                    .table(ClientMonthlyRequests::Table)
                    .if_not_exists()
                    .col(pk_auto(ClientMonthlyRequests::Id))
                    .col(uuid(ClientMonthlyRequests::Uuid).unique_key())
                    .col(big_integer(ClientMonthlyRequests::ClientId))
                    .col(big_integer(ClientMonthlyRequests::Requests))
                    .col(
                        date_time(ClientMonthlyRequests::DateAdded)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        date_time(ClientMonthlyRequests::LastUpdated)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(CLIENT_MONTHLY_REQUESTS_CLIENT_FK)
                            .from(
                                ClientMonthlyRequests::Table,
                                ClientMonthlyRequests::ClientId,
                            )
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
                    .table(SubscriptionPackage::Table)
                    .if_not_exists()
                    .col(pk_auto(SubscriptionPackage::Id))
                    .col(
                        uuid(SubscriptionPackage::Uuid)
                            .unique_key()
                            .default(Value::Uuid(default_uuid())),
                    )
                    .col(string(SubscriptionPackage::Name))
                    .col(double(SubscriptionPackage::Price))
                    .col(double(SubscriptionPackage::StorageCapacityMb))
                    .col(big_integer(SubscriptionPackage::MonthlyRequests))
                    .col(integer(SubscriptionPackage::MaxAllowedSessions))
                    .col(
                        date_time(SubscriptionPackage::DateAdded)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        date_time(SubscriptionPackage::LastUpdated)
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
                            .unique_key()
                            .default(Value::Uuid(default_uuid())),
                    )
                    .col(big_integer(ClientPackageSubscription::ClientId))
                    .col(big_integer(
                        ClientPackageSubscription::SubscriptionPackageId,
                    ))
                    .col(float(ClientPackageSubscription::Amount))
                    .col(
                        date_time(ClientPackageSubscription::DateAdded)
                            .default(Expr::current_timestamp()),
                    )
                    .col(date_time(ClientPackageSubscription::ExpiresAt))
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
                    .name(CLIENT_USAGE_UUID_INDEX)
                    .if_not_exists()
                    .table(ClientUsage::Table)
                    .col(ClientUsage::Uuid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(CLIENT_MONTHLY_REQUESTS_INDEX)
                    .if_not_exists()
                    .table(ClientMonthlyRequests::Table)
                    .col(ClientMonthlyRequests::Uuid)
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
                    .name(CLIENT_USAGE_UUID_INDEX)
                    .if_exists()
                    .table(ClientUsage::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name(CLIENT_MONTHLY_REQUESTS_INDEX)
                    .if_exists()
                    .table(ClientMonthlyRequests::Table)
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
pub enum ClientUsage {
    Table,
    Id,
    Uuid,
    ClientId,
    UsedStorageMb,
    ActiveSessions,
    DateAdded,
    LastUpdated,
}

#[derive(DeriveIden)]
pub enum ClientMonthlyRequests {
    Table,
    Id,
    Uuid,
    ClientId,
    Requests,
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
