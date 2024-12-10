use crate::apps::users::graphql::types::outputs::users::UserType;
use async_graphql::*;
use entity::entities::user;

#[derive(Default)]
pub struct UserQueries;

#[Object]
impl UserQueries {
    async fn user<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<UserType>> {
        let user = ctx.data::<Option<user::Model>>()?;
        if let Some(user) = user {
            Ok(Some(user.clone().into()))
        } else {
            Ok(None)
        }
    }
}
