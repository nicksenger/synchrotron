use std::i32;

use chrono::{DateTime, FixedOffset};
use graphql_client::{GraphQLQuery, QueryBody, Response as GraphQLResponse};
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

use crate::{messages::{ErrorPayload, application::{AllDocumentsRequestPayload, AllDocumentsSuccessPayload, CreateAnchorRequestPayload, CreateAnchorSuccessPayload, CreateUserAnchorSuccessPayload, DeleteAnchorRequestPayload, DeleteAnchorSuccessPayload, DocumentRequestPayload, DocumentSuccessPayload, JumpToAnchorRequestPayload, JumpToAnchorSuccessPayload, PageRequestPayload, PageSuccessPayload}, authentication::{
            LoginRequestPayload, LoginSuccessPayload, RegisterRequestPayload,
            RegisterSuccessPayload,
        }}, state::entities::{
        Anchor, Bookmark, Document as SchemaDocument, Page as SchemaPage, Track, User, UserAnchor,
    }};

const API_URL: &str = "https://synchrotron.nsenger.com/graphql";

type DateTimeFixedOffset = DateTime<FixedOffset>;

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

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gen/schema.json",
    query_path = "src/commands/operations.graphql"
)]
pub struct JumpToAnchor;

async fn graphQLRequest<T, U, V, W>(
    input: T,
    build_query: fn(U) -> QueryBody<U>,
    token: Option<String>,
) -> Result<W, ErrorPayload>
where
    T: Into<U> + Clone,
    U: Serialize,
    V: Into<W> + DeserializeOwned,
{
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(
        JsValue::from_serde(
            Some(&serde_json::to_string(&build_query(input.into())).unwrap()).unwrap(),
        )
        .ok()
        .as_ref(),
    );
    let request = Request::new_with_str(API_URL).unwrap();
    // Cannot set content-type header when using mode no-cors
    request
        .headers()
        .set("Content-Type", "application/json")
        .unwrap();
    request.headers().set("Accept", "application/json").unwrap();
    if let Some(t) = token {
        request.headers().set("Authorization", t.as_str()).unwrap();
    }
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request_and_init(&request, &opts))
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
    let response: GraphQLResponse<V> = json.into_serde().map_err(|e| ErrorPayload {
        content: "Failed to parse response".to_owned(),
    })?;

    if let Some(data) = response.data {
        Ok(data.into())
    } else {
        Err(ErrorPayload {
            content: "GraphQL error".to_owned(),
        })
    }
}

impl Into<i32> for login::UserRole {
    fn into(self) -> i32 {
        match self {
            login::UserRole::MODERATOR => 1,
            login::UserRole::ADMINISTRATOR => 2,
            _ => 0,
        }
    }
}

impl Into<login::Variables> for LoginRequestPayload {
    fn into(self) -> login::Variables {
        login::Variables {
            username: self.username.clone(),
            password: self.password.clone(),
        }
    }
}

impl Into<LoginSuccessPayload> for login::ResponseData {
    fn into(self) -> LoginSuccessPayload {
        LoginSuccessPayload {
            token: self.login.token,
            user: User {
                id: self.login.user.id as i32,
                username: self.login.user.username,
                role: self.login.user.role.into(),
            },
        }
    }
}

pub async fn login(
    input: LoginRequestPayload,
    token: Option<String>,
) -> Result<LoginSuccessPayload, ErrorPayload> {
    graphQLRequest::<LoginRequestPayload, login::Variables, login::ResponseData, LoginSuccessPayload>(input, Login::build_query, token).await
}

impl Into<i32> for register::UserRole {
    fn into(self) -> i32 {
        match self {
            register::UserRole::MODERATOR => 1,
            register::UserRole::ADMINISTRATOR => 2,
            _ => 0,
        }
    }
}

impl Into<register::Variables> for RegisterRequestPayload {
    fn into(self) -> register::Variables {
        register::Variables {
            username: self.username,
            password: self.password,
        }
    }
}

impl Into<RegisterSuccessPayload> for register::ResponseData {
    fn into(self) -> RegisterSuccessPayload {
        RegisterSuccessPayload {
            user: User {
                id: self.create_user.id as i32,
                username: self.create_user.username,
                role: self.create_user.role.into(),
            },
        }
    }
}

pub async fn register(
    input: RegisterRequestPayload,
    token: Option<String>,
) -> Result<RegisterSuccessPayload, ErrorPayload> {
    graphQLRequest::<
        RegisterRequestPayload,
        register::Variables,
        register::ResponseData,
        RegisterSuccessPayload,
    >(input, Register::build_query, token)
    .await
}

impl Into<all_documents::Variables> for AllDocumentsRequestPayload {
    fn into(self) -> all_documents::Variables {
        all_documents::Variables {
            document_limit: self.limit as i64,
            document_offset: self.offset as i64,
        }
    }
}

impl Into<AllDocumentsSuccessPayload> for all_documents::ResponseData {
    fn into(self) -> AllDocumentsSuccessPayload {
        AllDocumentsSuccessPayload {
            documents: self
                .documents
                .into_iter()
                .map(|d| SchemaDocument {
                    id: d.id as i32,
                    title: d.title,
                    created_at: d.created_at,
                    updated_at: d.updated_at,
                })
                .collect(),
        }
    }
}

pub async fn all_documents(
    input: AllDocumentsRequestPayload,
    token: Option<String>,
) -> Result<AllDocumentsSuccessPayload, ErrorPayload> {
    graphQLRequest::<
        AllDocumentsRequestPayload,
        all_documents::Variables,
        all_documents::ResponseData,
        AllDocumentsSuccessPayload,
    >(input, AllDocuments::build_query, token)
    .await
}

impl Into<document::Variables> for DocumentRequestPayload {
    fn into(self) -> document::Variables {
        document::Variables {
            document_id: self.document_id as i64,
            bookmark_limit: std::i32::MAX as i64,
            bookmark_offset: 0,
            page_limit: std::i32::MAX as i64,
            page_offset: 0,
            track_limit: std::i32::MAX as i64,
            track_offset: 0,
        }
    }
}

impl Into<DocumentSuccessPayload> for document::ResponseData {
    fn into(self) -> DocumentSuccessPayload {
        let document_id = self.document_by_id.id as i32;
        DocumentSuccessPayload {
            document: SchemaDocument {
                id: document_id,
                title: self.document_by_id.title,
                created_at: self.document_by_id.created_at,
                updated_at: self.document_by_id.updated_at,
            },
            bookmarks: self
                .document_by_id
                .bookmarks
                .into_iter()
                .map(|b| Bookmark {
                    id: b.id as i32,
                    title: b.title,
                    page_id: b.page.id as i32,
                    document_id,
                })
                .collect(),
            pages: self
                .document_by_id
                .pages
                .into_iter()
                .map(|p| SchemaPage {
                    id: p.id as i32,
                    page_number: p.page_number as i32,
                    image_path: p.image_path,
                    aspect_ratio: p.aspect_ratio as f32,
                    height: p.height as f32,
                    document_id,
                })
                .collect(),
            tracks: self
                .document_by_id
                .tracks
                .into_iter()
                .map(|t| Track {
                    id: t.id as i32,
                    track_number: t.track_number as i32,
                    title: t.title,
                    audio_path: t.audio_path,
                    document_id,
                })
                .collect(),
        }
    }
}

pub async fn document(
    input: DocumentRequestPayload,
    token: Option<String>,
) -> Result<DocumentSuccessPayload, ErrorPayload> {
    graphQLRequest::<
        DocumentRequestPayload,
        document::Variables,
        document::ResponseData,
        DocumentSuccessPayload,
    >(input, Document::build_query, token)
    .await
}

impl Into<page::Variables> for PageRequestPayload {
    fn into(self) -> page::Variables {
        page::Variables {
            page_id: self.page_id as i64,
        }
    }
}

impl Into<PageSuccessPayload> for page::ResponseData {
    fn into(self) -> PageSuccessPayload {
        let page_id = self.page_by_id.id as i32;
        PageSuccessPayload {
            page: SchemaPage {
                id: page_id,
                page_number: self.page_by_id.page_number as i32,
                image_path: self.page_by_id.image_path,
                aspect_ratio: self.page_by_id.aspect_ratio as f32,
                height: self.page_by_id.height as f32,
                document_id: self.page_by_id.document.id as i32,
            },
            anchors: self
                .page_by_id
                .anchors
                .into_iter()
                .map(|a| Anchor {
                    id: a.id as i32,
                    title: a.title,
                    track_time: a.track_time as f32,
                    position_top: a.position_top as f32,
                    position_left: a.position_left as f32,
                    page_id,
                    track_id: a.track.id as i32,
                    created_at: a.created_at,
                    updated_at: a.updated_at,
                })
                .collect(),
            user_anchors: self
                .page_by_id
                .user_anchors
                .into_iter()
                .map(|a| UserAnchor {
                    id: a.id as i32,
                    title: a.title,
                    track_time: a.track_time as f32,
                    position_top: a.position_top as f32,
                    position_left: a.position_left as f32,
                    page_id,
                    track_id: a.track.id as i32,
                    created_at: a.created_at,
                    updated_at: a.updated_at,
                    owner: a.owner.id as i32,
                })
                .collect(),
        }
    }
}

pub async fn page(
    input: PageRequestPayload,
    token: Option<String>,
) -> Result<PageSuccessPayload, ErrorPayload> {
    graphQLRequest::<PageRequestPayload, page::Variables, page::ResponseData, PageSuccessPayload>(
        input,
        Page::build_query,
        token,
    )
    .await
}

impl Into<create_anchor::Variables> for CreateAnchorRequestPayload {
    fn into(self) -> create_anchor::Variables {
        create_anchor::Variables {
            page_id: self.page_id as i64,
            position_left: self.position_left as f64,
            position_top: self.position_top as f64,
            title: self.title,
            track_id: self.track_id as i64,
            track_time: self.track_time as f64,
        }
    }
}

impl Into<create_user_anchor::Variables> for CreateAnchorRequestPayload {
    fn into(self) -> create_user_anchor::Variables {
        create_user_anchor::Variables {
            page_id: self.page_id as i64,
            position_left: self.position_left as f64,
            position_top: self.position_top as f64,
            title: self.title,
            track_id: self.track_id as i64,
            track_time: self.track_time as f64,
        }
    }
}

impl Into<CreateAnchorSuccessPayload> for create_anchor::ResponseData {
    fn into(self) -> CreateAnchorSuccessPayload {
        CreateAnchorSuccessPayload {
            anchor: Anchor {
                id: self.create_anchor.id as i32,
                title: self.create_anchor.title,
                track_time: self.create_anchor.track_time as f32,
                position_top: self.create_anchor.position_top as f32,
                position_left: self.create_anchor.position_left as f32,
                page_id: self.create_anchor.page.id as i32,
                track_id: self.create_anchor.track.id as i32,
                created_at: self.create_anchor.created_at,
                updated_at: self.create_anchor.updated_at,
            },
        }
    }
}

impl Into<CreateUserAnchorSuccessPayload> for create_user_anchor::ResponseData {
    fn into(self) -> CreateUserAnchorSuccessPayload {
        CreateUserAnchorSuccessPayload {
            user_anchor: UserAnchor {
                id: self.create_user_anchor.id as i32,
                title: self.create_user_anchor.title,
                track_time: self.create_user_anchor.track_time as f32,
                position_top: self.create_user_anchor.position_top as f32,
                position_left: self.create_user_anchor.position_left as f32,
                page_id: self.create_user_anchor.page.id as i32,
                track_id: self.create_user_anchor.track.id as i32,
                created_at: self.create_user_anchor.created_at,
                updated_at: self.create_user_anchor.updated_at,
                owner: self.create_user_anchor.owner.id as i32,
            },
        }
    }
}

pub async fn create_anchor(
    input: CreateAnchorRequestPayload,
    token: Option<String>,
) -> Result<CreateAnchorSuccessPayload, ErrorPayload> {
    graphQLRequest::<
        CreateAnchorRequestPayload,
        create_anchor::Variables,
        create_anchor::ResponseData,
        CreateAnchorSuccessPayload,
    >(input, CreateAnchor::build_query, token)
    .await
}

pub async fn create_user_anchor(
    input: CreateAnchorRequestPayload,
    token: Option<String>,
) -> Result<CreateUserAnchorSuccessPayload, ErrorPayload> {
    graphQLRequest::<
        CreateAnchorRequestPayload,
        create_user_anchor::Variables,
        create_user_anchor::ResponseData,
        CreateUserAnchorSuccessPayload,
    >(input, CreateUserAnchor::build_query, token)
    .await
}

impl Into<delete_anchor::Variables> for DeleteAnchorRequestPayload {
    fn into(self) -> delete_anchor::Variables {
        delete_anchor::Variables {
            anchor_id: self.anchor_id as i64,
        }
    }
}

impl Into<DeleteAnchorSuccessPayload> for delete_anchor::ResponseData {
    fn into(self) -> DeleteAnchorSuccessPayload {
        DeleteAnchorSuccessPayload {
            success: self.delete_anchor.success,
        }
    }
}

impl Into<delete_user_anchor::Variables> for DeleteAnchorRequestPayload {
    fn into(self) -> delete_user_anchor::Variables {
        delete_user_anchor::Variables {
            user_anchor_id: self.anchor_id as i64,
        }
    }
}

impl Into<DeleteAnchorSuccessPayload> for delete_user_anchor::ResponseData {
    fn into(self) -> DeleteAnchorSuccessPayload {
        DeleteAnchorSuccessPayload {
            success: self.delete_user_anchor.success,
        }
    }
}

pub async fn delete_anchor(
    input: DeleteAnchorRequestPayload,
    token: Option<String>,
) -> Result<DeleteAnchorSuccessPayload, ErrorPayload> {
    graphQLRequest::<
        DeleteAnchorRequestPayload,
        delete_anchor::Variables,
        delete_anchor::ResponseData,
        DeleteAnchorSuccessPayload,
    >(input, DeleteAnchor::build_query, token)
    .await
}

pub async fn delete_user_anchor(
    input: DeleteAnchorRequestPayload,
    token: Option<String>,
) -> Result<DeleteAnchorSuccessPayload, ErrorPayload> {
    graphQLRequest::<
        DeleteAnchorRequestPayload,
        delete_user_anchor::Variables,
        delete_user_anchor::ResponseData,
        DeleteAnchorSuccessPayload,
    >(input, DeleteUserAnchor::build_query, token)
    .await
}

impl Into<jump_to_anchor::Variables> for JumpToAnchorRequestPayload {
    fn into(self) -> jump_to_anchor::Variables {
        jump_to_anchor::Variables {
            anchor_id: self.anchor_id as i64,
        }
    }
}

impl Into<JumpToAnchorSuccessPayload> for jump_to_anchor::ResponseData {
    fn into(self) -> JumpToAnchorSuccessPayload {
        JumpToAnchorSuccessPayload {
            anchor: Anchor {
                id: self.anchor_by_id.id as i32,
                title: self.anchor_by_id.title,
                track_time: self.anchor_by_id.track_time as f32,
                position_top: self.anchor_by_id.position_top as f32,
                position_left: self.anchor_by_id.position_left as f32,
                page_id: self.anchor_by_id.page.id as i32,
                track_id: self.anchor_by_id.track.id as i32,
                created_at: self.anchor_by_id.created_at,
                updated_at: self.anchor_by_id.updated_at,
            },
        }
    }
}

pub async fn jump_to_anchor(
    input: JumpToAnchorRequestPayload,
    token: Option<String>,
) -> Result<JumpToAnchorSuccessPayload, ErrorPayload> {
    graphQLRequest::<
        JumpToAnchorRequestPayload,
        jump_to_anchor::Variables,
        jump_to_anchor::ResponseData,
        JumpToAnchorSuccessPayload,
    >(input, JumpToAnchor::build_query, token)
    .await
}
