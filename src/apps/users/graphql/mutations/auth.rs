use std::str::FromStr;

use async_graphql::*;
use entity::entities::{auth_token, user};
use sea_orm::{entity::*, sqlx::types::chrono, DatabaseConnection, QueryFilter, Set};
use uuid::Uuid;

use crate::apps::users::{
    graphql::types::{
        inputs::auth::{EmailPasswordSigninInput, EmailPasswordSignupInput},
        outputs::users::{AuthTokenType, UserType},
    },
    utils::auth::create_user_auth_token,
};

#[derive(Default)]
pub struct UsersAuthMutations;

#[Object]
impl UsersAuthMutations {
    async fn email_password_signup<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: EmailPasswordSignupInput,
    ) -> Result<UserType> {
        let db = ctx.data::<DatabaseConnection>()?;
        if input.password1 != input.password2 {
            return Err(Error::new("Passwords do not match"));
        }
        let password_hash = bcrypt::hash(input.password1, bcrypt::DEFAULT_COST)?;
        let new_user = user::ActiveModel {
            email: Set(input.email),
            uuid: Set(Uuid::new_v4()),
            password_hash: Set(Some(password_hash)),
            ..Default::default()
        };
        let new_user: user::Model = new_user.insert(db).await?;
        Ok(new_user.into())
    }

    async fn email_password_signin<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: EmailPasswordSigninInput,
    ) -> Result<AuthTokenType> {
        let db = ctx.data::<DatabaseConnection>()?;

        let user = user::Entity::find()
            .filter(user::Column::Email.eq(input.email))
            .one(db)
            .await?;

        if let Some(user) = user {
            if let Some(password_hash) = &user.password_hash {
                if bcrypt::verify(input.password, password_hash.as_str())? {
                    let tokens = auth_token::Entity::find()
                        .filter(auth_token::Column::UserId.eq(user.id as i64))
                        .all(db)
                        .await?;
                    for token in tokens {
                        token.delete(db).await?;
                    }
                    let new_token = create_user_auth_token(&user, db).await?;
                    return Ok(new_token.into());
                } else {
                    return Err(Error::new("Incorrect email or password"));
                }
            } else {
                return Err(Error::new("Password login not found"));
            }
        } else {
            return Err(Error::new("User with that email not found"));
        }
    }

    async fn refresh_token<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        refresh_token: String,
    ) -> Result<AuthTokenType> {
        let db = ctx.data::<DatabaseConnection>()?;

        let auth_token = auth_token::Entity::find()
            .filter(auth_token::Column::Uuid.eq(Uuid::from_str(refresh_token.as_str())?))
            .one(db)
            .await?;
        if let Some(token) = auth_token {
            if token.expires_at <= chrono::Utc::now().naive_utc() {
                token.delete(db).await?;
                return Err(Error::new("Token expired"));
            } else {
                let user = user::Entity::find_by_id(token.user_id as i32)
                    .one(db)
                    .await?;

                if let Some(user) = user {
                    token.delete(db).await?;

                    let new_token = create_user_auth_token(&user, db).await?;
                    Ok(new_token.into())
                } else {
                    return Err(Error::new("Invalid refresh_token"));
                }
            }
        } else {
            return Err(Error::new("Invalid refresh_token"));
        }
    }
}
