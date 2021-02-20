use super::schema::Context;
use crate::entities::{
    Anchor, Bookmark, CreateAnchor, CreateUserAnchor, DeleteAnchor, DeleteAnchorResponse,
    DeleteBookmarkResponse, DeleteUserAnchor, DeleteUserAnchorResponse, Login, LoginResponse,
    NewUser, Track, UpdateUserRole, UpdateUserRoleResponse, User, UserAnchor,
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

    pub async fn create_user_anchor(
        ctx: &Context,
        data: CreateUserAnchor,
    ) -> FieldResult<UserAnchor> {
        let response = ctx
            .user_anchor_data
            .as_ref()
            .unwrap()
            .create_user_anchor(ctx.user.clone(), data)
            .await?;
        Ok(response)
    }

    pub async fn delete_user_anchor(
        ctx: &Context,
        data: DeleteUserAnchor,
    ) -> FieldResult<DeleteUserAnchorResponse> {
        let response = ctx
            .user_anchor_data
            .as_ref()
            .unwrap()
            .delete_user_anchor(ctx.user.clone(), data)
            .await?;
        Ok(response)
    }

    pub async fn update_track_title(
        ctx: &Context,
        track_id: i32,
        title: String,
    ) -> FieldResult<Track> {
        let response = ctx
            .track_data
            .as_ref()
            .unwrap()
            .update_track_title(track_id, title, ctx.user.clone())
            .await?;
        Ok(response)
    }

    pub async fn create_bookmark(
        ctx: &Context,
        title: String,
        page_id: i32,
        document_id: i32,
    ) -> FieldResult<Bookmark> {
        let response = ctx
            .bookmark_data
            .as_ref()
            .unwrap()
            .create_bookmark(title, page_id, document_id, ctx.user.clone())
            .await?;
        Ok(response)
    }

    pub async fn delete_bookmark(
        ctx: &Context,
        bookmark_id: i32,
    ) -> FieldResult<DeleteBookmarkResponse> {
        let response = ctx
            .bookmark_data
            .as_ref()
            .unwrap()
            .delete_bookmark(bookmark_id, ctx.user.clone())
            .await?;
        Ok(response)
    }
}
