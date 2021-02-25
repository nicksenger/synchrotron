use graphql_client::{GraphQLQuery, Response as GraphQLResponse};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::messages::{
    authentication::{LoginRequestPayload, LoginSuccessPayload},
    ErrorPayload,
};

const API_URL: &str = "https://synchrotron.nsenger.com/graphql";

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/commands/operations.graphql"
)]
pub struct Register;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/commands/operations.graphql"
)]
pub struct Login;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/commands/operations.graphql"
)]
pub struct AllDocuments;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/commands/operations.graphql"
)]
pub struct Document;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/commands/operations.graphql"
)]
pub struct Page;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/commands/operations.graphql"
)]
pub struct CreateAnchor;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/commands/operations.graphql"
)]
pub struct DeleteAnchor;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/commands/operations.graphql"
)]
pub struct CreateUserAnchor;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/commands/operations.graphql"
)]
pub struct DeleteUserAnchor;

impl Into<login::Variables> for &LoginRequestPayload {
    fn into(self) -> login::Variables {
        login::Variables {
            username: self.username.clone(),
            password: self.password.clone(),
        }
    }
}

pub async fn login(input: login::Variables) -> Result<LoginSuccessPayload, ErrorPayload> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(
        JsValue::from_serde(Some(&Login::build_query(input)).unwrap())
            .ok()
            .as_ref(),
    );
    let request =
        Request::new_with_request_and_init(&Request::new_with_str(API_URL).unwrap(), &opts)
            .unwrap();
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| ErrorPayload {
            content: "Failed to generate JS future from request".to_owned(),
        })?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json().unwrap())
        .await
        .map_err(|e| ErrorPayload {
            content: "Failed to generate JS future parsing JSON".to_owned(),
        })?;
    let response: GraphQLResponse<login::ResponseData> =
        json.into_serde().map_err(|e| ErrorPayload {
            content: "Failed to parse response".to_owned(),
        })?;

    if let Some(data) = response.data {
        Ok(LoginSuccessPayload {
            user_id: data.login.user.id as i32,
            token: data.login.token,
        })
    } else {
        Err(ErrorPayload {
            content: "GraphQL error".to_owned(),
        })
    }
}
