use juniper::FieldResult;

use super::schema::Context;
use crate::entities::{Anchor, Document, Page, User};

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn user_by_id(ctx: &Context, id: i32) -> FieldResult<User> {
        Ok(ctx.user_data.as_ref().unwrap().user_by_id(id).await)
    }

    async fn users(ctx: &Context) -> FieldResult<Vec<User>> {
        Ok(ctx.user_data.as_ref().unwrap().all_users().await?)
    }

    async fn document_by_id(ctx: &Context, id: i32) -> FieldResult<Document> {
        Ok(ctx
            .document_data
            .as_ref()
            .unwrap()
            .documents_by_id(id)
            .await)
    }

    async fn documents(ctx: &Context, limit: i32, offset: i32) -> FieldResult<Vec<Document>> {
        Ok(ctx
            .document_data
            .as_ref()
            .unwrap()
            .all_documents(limit, offset)
            .await?)
    }

    async fn page_by_id(ctx: &Context, id: i32) -> FieldResult<Page> {
        Ok(ctx.page_data.as_ref().unwrap().pages_by_id(id).await)
    }

    async fn anchor_by_id(ctx: &Context, id: i32) -> FieldResult<Anchor> {
        Ok(ctx.anchor_data.as_ref().unwrap().anchors_by_id(id).await)
    }
}
