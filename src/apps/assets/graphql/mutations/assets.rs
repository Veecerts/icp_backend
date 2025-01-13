use std::str::FromStr;

use async_graphql::*;
use chrono::Utc;
use entity::entities::{
    asset, client, client_package_subscription, client_usage, folder, subscription_package, user,
};
use sea_orm::{
    entity::*, DatabaseConnection, EntityTrait, JoinType, PaginatorTrait, QueryFilter, QuerySelect,
};
use uuid::Uuid;

use crate::apps::assets::{
    graphql::types::{
        inputs::assets::{AssetInput, FolderInput},
        outputs::assets::{AssetType, FolderType},
    },
    utils::{
        contract::{Contract, CreateNFTResult, MintNFTResult},
        files::bytes_to_mb,
        formating::format_id,
        pinata::Pinata,
    },
};

#[derive(Default)]
pub struct AssetMutations;

#[Object]
impl AssetMutations {
    async fn create_update_folder<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: FolderInput,
    ) -> Result<FolderType> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user = ctx.data::<Option<user::Model>>()?;

        if let Some(user) = user {
            let client = client::Entity::find()
                .filter(client::Column::UserId.eq(user.id))
                .one(db)
                .await?;

            if let Some(client) = client {
                if let Some(uuid) = input.uuid {
                    let folder = folder::Entity::find()
                        .filter(folder::Column::Uuid.eq(Uuid::from_str(uuid.to_string().as_str())?))
                        .one(db)
                        .await?;
                    if let Some(folder) = folder {
                        if folder.client_id != client.id as i64 {
                            return Err(Error::new(
                                "You are not authorized to perform this action",
                            ));
                        }
                        let mut folder: folder::ActiveModel = folder.into();
                        folder.name = Set(input.name);
                        folder.description = Set(input.description);
                        folder.last_updated = Set(Utc::now().naive_utc());

                        let folder = folder.update(db).await?;
                        Ok(folder.into())
                    } else {
                        Err(Error::new(format!(
                            "Folder with uuid {} was not found",
                            *uuid
                        )))
                    }
                } else {
                    let value = input.logo.value(ctx)?;
                    if let Some(content_type) = value.content_type {
                        if !content_type.starts_with("image") {
                            return Err(Error::new("Please provide a valid image"));
                        }

                        let pinata_res = Pinata::pin_file(value.content).await?;
                        let logo_url = Some(Pinata::build_url(pinata_res.ipfs_hash.clone()));
                        let count = folder::Entity::find().count(db).await?;
                        let symbol = format_id(count + 1);

                        let result = Contract::create_nft(
                            &input.name,
                            &symbol,
                            &input.description,
                            &logo_url,
                        )
                        .await?;

                        if let CreateNFTResult::Ok(res) = result {
                            let folder = folder::ActiveModel {
                                id: Set(res.1.id as i32),
                                uuid: Set(Uuid::new_v4()),
                                name: Set(input.name),
                                logo_hash: Set(pinata_res.ipfs_hash),
                                description: Set(input.description),
                                client_id: Set(client.id as i64),
                                ..Default::default()
                            };
                            let folder = folder.insert(db).await?;
                            Ok(folder.into())
                        } else if let CreateNFTResult::Err(err) = result {
                            return Err(Error::new(format!("Contract error: {}", err)));
                        } else {
                            return Err(Error::new("Failed to create NFT"));
                        }
                    } else {
                        Err(Error::new("Unable to verify image type"))
                    }
                }
            } else {
                Err(Error::new(
                    "You do not currently have an active subscription",
                ))
            }
        } else {
            Err(Error::new(
                "You must be authenticated to perform this action",
            ))
        }
    }

    async fn create_update_asset<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: AssetInput,
    ) -> Result<AssetType> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user = ctx.data::<Option<user::Model>>()?;
        let folder = folder::Entity::find()
            .filter(folder::Column::Uuid.eq(Uuid::from_str(input.folder_uuid.as_str())?))
            .one(db)
            .await?;

        if let Some(folder) = folder {
            if let Some(user) = user {
                let user_client = client::Entity::find()
                    .filter(client::Column::UserId.eq(user.id))
                    .one(db)
                    .await?;

                if user_client.is_none() {
                    return Err(Error::new("User Client not found"));
                }

                let client_package = client::Entity::find()
                    .filter(client::Column::UserId.eq(user.id))
                    .join(
                        JoinType::InnerJoin,
                        client::Relation::ClientPackageSubscription.def(),
                    )
                    .join(
                        JoinType::InnerJoin,
                        client_package_subscription::Relation::SubscriptionPackage.def(),
                    )
                    .select_also(subscription_package::Entity)
                    .one(db)
                    .await?;

                if let Some(client_package) = client_package {
                    let user_client = client_package.0;

                    let client_usage = match client_usage::Entity::find()
                        .filter(client_usage::Column::ClientId.eq(user_client.id))
                        .one(db)
                        .await?
                    {
                        Some(usage) => usage,
                        None => {
                            client_usage::ActiveModel {
                                uuid: Set(Uuid::new_v4()),
                                client_id: Set(user_client.id.into()),
                                used_storage_mb: Set(0.0),
                                active_sessions: Set(0),
                                ..Default::default()
                            }
                            .insert(db)
                            .await?
                        }
                    };

                    if let Some(package) = client_package.1 {
                        let file_value = input.file.value(ctx)?;
                        let file_size = file_value.size()?;
                        let size_in_mb = bytes_to_mb(file_size);

                        if let Some(uuid) = input.uuid {
                            let asset = asset::Entity::find()
                                .filter(
                                    asset::Column::Uuid
                                        .eq(Uuid::from_str(uuid.to_string().as_str())?),
                                )
                                .one(db)
                                .await?;

                            if let Some(asset) = asset {
                                if asset.client_id != user_client.id as i64 {
                                    return Err(Error::new(
                                        "You are not authorized to perform this action",
                                    ));
                                }
                                let new_used_storage_mb =
                                    client_usage.used_storage_mb - asset.size_mb + size_in_mb;
                                if new_used_storage_mb > package.storage_capacity_mb {
                                    return Err(
                                        Error::new(format!("Insuficient storage: Uploading file of {}mb will exceed your maximum storage of {}mb.", size_in_mb, package.storage_capacity_mb))
                                    );
                                }

                                Pinata::unpin_file(&asset.ipfs_hash).await?;
                                Contract::burn_nft(format!("{}x{}", asset.nft_id, asset.folder_id))
                                    .await?;

                                let pin_result = Pinata::pin_file(file_value.content).await?;
                                let result = Contract::mint_nft(
                                    folder.id as u64,
                                    &asset.uuid.to_string(),
                                    &pin_result.ipfs_hash,
                                )
                                .await?;

                                if let MintNFTResult::Ok(res) = result {
                                    let mut asset: asset::ActiveModel = asset.into();
                                    asset.nft_id = Set(res.1.id as i64);
                                    asset.size_mb = Set(size_in_mb);
                                    asset.ipfs_hash = Set(pin_result.ipfs_hash);
                                    asset.folder_id = Set(folder.id.into());
                                    asset.name = Set(input.name);
                                    asset.description = Set(input.description);
                                    asset.last_updated = Set(Utc::now().naive_utc());
                                    if let Some(content_type) = file_value.content_type {
                                        asset.content_type = Set(content_type)
                                    } else {
                                        return Err(Error::new("Failed to identify content_type"));
                                    }

                                    let asset = asset.update(db).await?;
                                    Ok(asset.into())
                                } else if let MintNFTResult::Err(err) = result {
                                    return Err(Error::new(format!("Contract error: {}", err)));
                                } else {
                                    return Err(Error::new("Failed to mint nft"));
                                }
                            } else {
                                Err(Error::new(format!(
                                    "Entity with uuid {} was not found",
                                    &uuid.to_string()
                                )))
                            }
                        } else {
                            let new_used_storage_mb = client_usage.used_storage_mb + size_in_mb;
                            if new_used_storage_mb > package.storage_capacity_mb {
                                return Err(
                                    Error::new(format!("Insuficient storage: Uploading file of {}mb will exceed your maximum storage of {}mb.", size_in_mb, package.storage_capacity_mb))
                                );
                            }

                            let uuid = Uuid::new_v4();
                            let pinata_res = Pinata::pin_file(file_value.content).await?;
                            let result = Contract::mint_nft(
                                folder.id as u64,
                                &uuid.to_string(),
                                &pinata_res.ipfs_hash,
                            )
                            .await?;

                            if let MintNFTResult::Ok(res) = result {
                                if let Some(content_type) = file_value.content_type {
                                    let new_asset = asset::ActiveModel {
                                        uuid: Set(uuid),
                                        name: Set(input.name),
                                        description: Set(input.description),
                                        folder_id: Set(folder.id.into()),
                                        nft_id: Set(res.1.id as i64),
                                        client_id: Set(user_client.id as i64),
                                        ipfs_hash: Set(pinata_res.ipfs_hash),
                                        size_mb: Set(size_in_mb),
                                        content_type: Set(content_type),
                                        ..Default::default()
                                    };
                                    let new_asset = new_asset.insert(db).await?;
                                    Ok(new_asset.into())
                                } else {
                                    Err(Error::new("Failed to identify content_type"))
                                }
                            } else if let MintNFTResult::Err(err) = result {
                                return Err(Error::new(format!("Contract error: {}", err)));
                            } else {
                                return Err(Error::new("Failed to mint nft"));
                            }
                        }
                    } else {
                        Err(Error::new(
                            "You do not currently have an active subscription",
                        ))
                    }
                } else {
                    Err(Error::new("You are not authorized to perform this action"))
                }
            } else {
                Err(Error::new(
                    "You must be authenticated to perform this action",
                ))
            }
        } else {
            Err(Error::new(format!(
                "Folder with uuid {} was not found",
                input.folder_uuid
            )))
        }
    }
}
