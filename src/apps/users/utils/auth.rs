use async_graphql::*;
use entity::entities::{auth_token, user};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use sea_orm::{entity::*, DatabaseConnection, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::settings::ENV;

#[derive(Debug, Deserialize, Serialize)]
pub enum JWTVariant {
    User(String),
    Client(i32),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomJWTClaims {
    varaint: JWTVariant,
    exp: usize,
    iat: usize,  // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    sub: String,
}

pub async fn create_user_auth_token(
    user: &user::Model,
    db: &DatabaseConnection,
) -> Result<auth_token::Model> {
    let expires_at = chrono::Utc::now() + std::time::Duration::from_secs_f32(24.0 * 60.0 * 60.0);

    let custom_claims = CustomJWTClaims {
        varaint: JWTVariant::User(user.email.clone()),
        iat: chrono::Utc::now().timestamp() as usize,
        exp: expires_at.clone().timestamp() as usize,
        iss: String::from("veecerts"),
        sub: String::from("User Token"),
    };
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &custom_claims,
        &EncodingKey::from_secret(ENV::init().secret_key.as_ref()),
    )?;

    let new_token = auth_token::ActiveModel {
        token: Set(token),
        uuid: Set(Uuid::new_v4()),
        user_id: Set(user.id as i64),
        expires_at: Set(expires_at.naive_utc()),
        ..Default::default()
    };
    let new_token: auth_token::Model = new_token.insert(db).await?;
    Ok(new_token)
}

pub async fn decode_user_auth_token(
    authorization: String,
    db: &DatabaseConnection,
) -> Result<Option<user::Model>> {
    if let Some(parts) = authorization.split_once(" ") {
        let token = jsonwebtoken::decode::<CustomJWTClaims>(
            parts.1,
            &DecodingKey::from_secret(ENV::init().secret_key.as_ref()),
            &Validation::default(),
        )?;
        if let JWTVariant::User(email) = token.claims.varaint {
            let user = user::Entity::find()
                .filter(user::Column::Email.eq(email))
                .one(db)
                .await?;
            Ok(user)
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
