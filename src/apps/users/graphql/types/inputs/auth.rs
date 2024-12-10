use async_graphql::*;

#[derive(InputObject)]
pub struct EmailPasswordSignupInput {
    pub email: String,
    pub password1: String,
    pub password2: String,
}

#[derive(InputObject)]
pub struct EmailPasswordSigninInput {
    pub email: String,
    pub password: String,
}
