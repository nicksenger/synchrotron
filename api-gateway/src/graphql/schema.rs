use juniper::EmptySubscription;

use super::{mutation::Mutation, query::Query};
use crate::data::{BookmarkData, DocumentData, PageData, TrackData, UserData};
use schema::shared::User;

#[derive(Clone)]
pub struct Context {
    pub user: Option<User>,
    pub user_data: Option<UserData>,
    pub document_data: Option<DocumentData>,
    pub bookmark_data: Option<BookmarkData>,
    pub page_data: Option<PageData>,
    pub track_data: Option<TrackData>,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new(
        user: Option<User>,
        user_data: Option<UserData>,
        document_data: Option<DocumentData>,
        bookmark_data: Option<BookmarkData>,
        page_data: Option<PageData>,
        track_data: Option<TrackData>,
    ) -> Self {
        Self {
            user,
            user_data,
            document_data,
            bookmark_data,
            page_data,
            track_data,
        }
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, juniper::EmptySubscription::new())
}
