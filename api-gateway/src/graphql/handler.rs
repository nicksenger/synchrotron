use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Json},
    Error, HttpRequest, HttpResponse,
};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use super::schema::Context;
use crate::{data::UserData, AppData};

pub async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn graphql(
    req: HttpRequest,
    st: Data<AppData>,
    data: Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user_data = UserData::new(st.user_channel.clone());

    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .unwrap_or("");
    let user_id = user_data.verify(token.to_owned()).await;

    log::info!(
        "Processing request for user \"{}\".",
        user_id
            .map(|id| format!("{}", id))
            .unwrap_or("Anonymous".to_owned())
    );

    let ctx = Context::new(user_id, user_data);
    let res = data.execute(&st.schema, &ctx).await;
    let json = serde_json::to_string(&res).map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}
