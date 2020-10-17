use std::sync::Arc;

use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Json},
    Error, HttpResponse,
};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use super::schema::{Context, Schema};
use crate::data::{UserData};

pub async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn graphql(
    st: Data<Arc<Schema>>,
    data: Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user_data = UserData::new();

    let ctx = Context::new(user_data);
    let res = data.execute(&st, &ctx).await;
    let json = serde_json::to_string(&res).map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}
