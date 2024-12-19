use async_graphql::*;
use entity::entities::{auth_token, client, profile, user};
use sea_orm::{entity::*, DatabaseConnection, EntityTrait, QueryFilter};

use super::clients::ClientType;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct UserType {
    pub id: ID,
    pub uuid: String,
    pub email: String,
    pub wallet_address: Option<String>,
    pub date_added: String,
    pub last_updated: String,
}

impl From<user::Model> for UserType {
    fn from(value: user::Model) -> Self {
        Self {
            id: value.id.into(),
            uuid: value.uuid.into(),
            email: value.email,
            wallet_address: value.wallet_address,
            date_added: value.date_added.to_string(),
            last_updated: value.last_updated.to_string(),
        }
    }
}

#[ComplexObject]
impl UserType {
    async fn client<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<ClientType>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user_id = self.id.parse::<i64>()?;
        let client = client::Entity::find()
            .filter(client::Column::UserId.eq(user_id))
            .one(db)
            .await?;

        if let Some(client) = client {
            Ok(Some(client.into()))
        } else {
            Ok(None)
        }
    }

    async fn profile<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<ProfileType>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user_id = self.id.parse::<i64>()?;
        let profile = profile::Entity::find()
            .filter(profile::Column::UserId.eq(user_id))
            .one(db)
            .await?;

        if let Some(profile) = profile {
            Ok(Some(profile.into()))
        } else {
            Ok(None)
        }
    }
}

#[derive(SimpleObject)]
pub struct ProfileType {
    pub id: ID,
    pub uuid: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub image_hash: Option<String>,

    #[graphql(skip)]
    pub user_id: i64,

    pub date_added: String,
    pub last_updated: String,
}

impl From<profile::Model> for ProfileType {
    fn from(value: profile::Model) -> Self {
        Self {
            id: value.id.into(),
            uuid: value.uuid.to_string(),
            first_name: value.first_name,
            last_name: value.last_name,
            image_hash: value.image_hash,
            user_id: value.user_id,
            date_added: value.date_added.to_string(),
            last_updated: value.last_updated.to_string(),
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct AuthTokenType {
    pub id: ID,
    pub token: String,
    pub date_added: String,
    pub expires_at: String,
    pub refresh_token: String,

    #[graphql(skip)]
    pub user_id: i64,
}

#[ComplexObject]
impl AuthTokenType {
    async fn user<'ctx>(&self, ctx: &Context<'ctx>) -> Result<UserType> {
        let db = ctx.data::<DatabaseConnection>()?;

        let user = user::Entity::find_by_id(self.user_id as i32)
            .one(db)
            .await?;
        if let Some(user) = user {
            return Ok(user.into());
        } else {
            return Err(Error::new("AuthToken User not found"));
        }
    }
}

impl From<auth_token::Model> for AuthTokenType {
    fn from(value: auth_token::Model) -> Self {
        Self {
            id: value.id.into(),
            token: value.token,
            date_added: value.date_added.to_string(),
            expires_at: value.expires_at.to_string(),
            refresh_token: value.uuid.into(),
            user_id: value.user_id,
        }
    }
}
