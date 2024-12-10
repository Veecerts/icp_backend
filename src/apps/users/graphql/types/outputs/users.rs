use async_graphql::*;
use entity::entities::{auth_token, user};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(SimpleObject)]
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
