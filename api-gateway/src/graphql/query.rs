use juniper::FieldResult;

use super::schema::Context;
use crate::entities::{Document, User};

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
}
