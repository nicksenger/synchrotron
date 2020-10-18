use super::schema::Context;
use crate::entities::{User, NewUser};
use juniper::FieldResult;

pub struct Mutation {}

#[juniper::graphql_object(Context = Context)]
impl Mutation {
  pub async fn create_user(ctx: &Context, data: NewUser) -> FieldResult<User> {
    Ok(ctx.user_data.create_user(data).await)
  }
}
