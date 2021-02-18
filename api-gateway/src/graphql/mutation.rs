use super::schema::Context;
use crate::entities::{
    Anchor, CreateAnchor, DeleteAnchor, DeleteAnchorResponse, Login, LoginResponse, NewUser,
    UpdateUserRole, UpdateUserRoleResponse, User,
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

    pub async fn create_anchor(ctx: &Context, data: CreateAnchor) -> FieldResult<Anchor> {
        let response = ctx
            .anchor_data
            .as_ref()
            .unwrap()
            .create_anchor(ctx.user.clone(), data)
            .await?;
        Ok(response)
    }

    pub async fn delete_anchor(
        ctx: &Context,
        data: DeleteAnchor,
    ) -> FieldResult<DeleteAnchorResponse> {
        let response = ctx
            .anchor_data
            .as_ref()
            .unwrap()
            .delete_anchor(ctx.user.clone(), data)
            .await?;
        Ok(response)
    }
}
