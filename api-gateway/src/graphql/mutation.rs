use super::schema::Context;
use crate::entities::{
    Login, LoginResponse, NewUser, UpdateUserRole, UpdateUserRoleResponse, User,
};
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

    pub async fn update_user_role(
        ctx: &Context,
        data: UpdateUserRole,
    ) -> FieldResult<UpdateUserRoleResponse> {
        let response = ctx
            .user_data
            .as_ref()
            .unwrap()
            .update_user_role(data, ctx.user.clone())
            .await?;
        Ok(UpdateUserRoleResponse {
            success: response.success,
        })
    }
}
