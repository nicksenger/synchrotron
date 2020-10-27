use std::rc::Rc;

use futures::{future::ready, FutureExt, Stream, StreamExt};
use graphql_client::{GraphQLQuery, Response};

use crate::{
    LoginFailedPayload, LoginPayload, LoginSuccessPayload, Msg, RegisterFailedPayload,
    RegisterPayload, RegisterSuccessPayload, Route,
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/operations.graphql"
)]
pub struct Register;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/operations.graphql"
)]
pub struct Login;

pub async fn login(payload: LoginPayload) -> Msg {
    let client = reqwest::Client::new();
    if let Ok(res) = client
        .post("http://localhost:8000/graphql")
        .json(&Login::build_query(login::Variables {
            username: payload.username.clone(),
            password: payload.password,
        }))
        .send()
        .await
    {
        if let Ok(response) = res.json::<Response<login::ResponseData>>().await {
            if let Some(data) = response.data {
                Msg::LoginSuccess(LoginSuccessPayload {
                    username: payload.username,
                    token: data.login.token,
                })
            } else {
                Msg::LoginFailed(LoginFailedPayload {
                    message: "GraphQL error".to_owned(),
                })
            }
        } else {
            Msg::LoginFailed(LoginFailedPayload {
                message: "Failed to parse response as JSON".to_owned(),
            })
        }
    } else {
        Msg::LoginFailed(LoginFailedPayload {
            message: "Request failed".to_owned(),
        })
    }
}

pub async fn register(payload: RegisterPayload) -> Msg {
    let client = reqwest::Client::new();
    if let Ok(res) = client
        .post("http://localhost:8000/graphql")
        .json(&Register::build_query(register::Variables {
            username: payload.username,
            password: payload.password,
        }))
        .send()
        .await
    {
        if let Ok(response) = res.json::<Response<register::ResponseData>>().await {
            if let Some(data) = response.data {
                Msg::RegisterSuccess(RegisterSuccessPayload {
                    user_id: data.create_user.id as i32,
                })
            } else {
                Msg::RegisterFailed(RegisterFailedPayload {
                    message: "GraphQL error".to_owned(),
                })
            }
        } else {
            Msg::RegisterFailed(RegisterFailedPayload {
                message: "Failed to parse response as JSON".to_owned(),
            })
        }
    } else {
        Msg::RegisterFailed(RegisterFailedPayload {
            message: "Request failed".to_owned(),
        })
    }
}
