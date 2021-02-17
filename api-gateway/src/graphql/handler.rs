use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Json},
    Error, HttpRequest, HttpResponse,
};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use super::schema::Context;
use crate::{
    data::{BookmarkData, DocumentData, PageData, UserData, TrackData},
    AppData,
};

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
    let document_data = DocumentData::new(st.courses_channel.clone());
    let bookmark_data = BookmarkData::new(st.courses_channel.clone());
    let page_data = PageData::new(st.courses_channel.clone());
    let track_data = TrackData::new(st.courses_channel.clone());

    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    let user = if token.is_some() {
        user_data.authenticate(token.unwrap().to_owned()).await.ok()
    } else {
        None
    };

    log::info!(
        "Processing request for user \"{}\".",
        user.as_ref()
            .map(|u| format!("{}", u.username))
            .unwrap_or("Anonymous".to_owned())
    );

    let ctx = Context::new(
        user,
        Some(user_data),
        Some(document_data),
        Some(bookmark_data),
        Some(page_data),
        Some(track_data)
    );
    let res = data.execute(&st.schema, &ctx).await;
    let json = serde_json::to_string(&res).map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}
