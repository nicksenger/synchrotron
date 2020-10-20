use std::sync::Arc;

use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Json},
    Error, HttpRequest, HttpResponse,
};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use super::schema::{Context, Schema};
use crate::data::UserData;

pub async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn graphql(
    req: HttpRequest,
    st: Data<(Arc<Schema>, String, tonic::transport::Channel)>,
    data: Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user_data = UserData::new(st.2.clone());

    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .unwrap_or("");
    let user_id = user_data.verify(token.to_owned()).await;

    let ctx = Context::new(user_id, user_data);
    let res = data.execute(&st.0, &ctx).await;
    let json = serde_json::to_string(&res).map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}
