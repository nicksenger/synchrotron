use super::schema::Context;
use crate::entities::{Login, LoginResponse, NewUser, User};
use juniper::FieldResult;

pub struct Mutation {}

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    pub async fn create_user(ctx: &Context, data: NewUser) -> FieldResult<User> {
        Ok(ctx.user_data.as_ref().unwrap().create_user(data).await?)
    }

    pub async fn login(ctx: &Context, data: Login) -> FieldResult<LoginResponse> {
        let token = ctx.user_data.as_ref().unwrap().login(data).await?;
        Ok(LoginResponse { token })
    }
}
