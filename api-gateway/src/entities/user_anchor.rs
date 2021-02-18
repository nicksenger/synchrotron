use std::convert::From;

use chrono::{DateTime, FixedOffset};

use super::{Page, Track, User};
use crate::graphql::schema::Context;

#[derive(Debug, Clone)]
/// UserAnchor for a page
pub struct UserAnchor {
    // ID of the anchor
    pub id: i32,
    // Title of the anchor
    pub title: String,
    // Point in the track that the anchor points to
    pub track_time: f64,
    // Y position for the anchor in the document
    pub position_top: f64,
    // X position for the anchor in the document
    pub position_left: f64,
    // ID of the page the anchor is on
    pub page_id: i32,
    // ID of the track that this anchor refers to
    pub track_id: i32,
    // Date that this anchor was created
    pub created_at: String,
    // Date that this anchor was last updated
    pub updated_at: String,
    // Owner of the anchor
    pub owner_id: i32,
}

#[derive(juniper::GraphQLInputObject, Debug, Clone)]
// Retrieving uesr anchors for a page
pub struct PageUserAnchors {
    // Page to query for user anchors
    pub page_id: i32,
}

#[derive(juniper::GraphQLInputObject, Debug, Clone)]
pub struct CreateUserAnchor {
    // Title for the new anchor
    pub title: String,
    // Track time that the new anchor will point to
    pub track_time: f64,
    // Y position of the anchor on page
    pub position_top: f64,
    // X position of the anchor on page
    pub position_left: f64,
    // ID of the page for the anchor
    pub page_id: i32,
    // Track for the anchor
    pub track_id: i32,
}

#[derive(juniper::GraphQLInputObject, Debug, Clone)]
pub struct DeleteUserAnchor {
    // ID of the anchor to delete
    pub anchor_id: i32,
}

#[derive(juniper::GraphQLObject, Debug, Clone)]
pub struct DeleteUserAnchorResponse {
    // Indicates whether deletion was successful
    pub success: bool,
}

#[juniper::graphql_object(Context = Context)]
impl UserAnchor {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn track_time(&self) -> f64 {
        self.track_time
    }

    pub fn position_top(&self) -> f64 {
        self.position_top
    }

    pub fn position_left(&self) -> f64 {
        self.position_left
    }

    pub async fn page(&self, context: &Context) -> Page {
        context
            .page_data
            .as_ref()
            .unwrap()
            .pages_by_id(self.page_id)
            .await
    }

    pub async fn track(&self, context: &Context) -> Track {
        context
            .track_data
            .as_ref()
            .unwrap()
            .tracks_by_id(self.track_id)
            .await
    }

    pub fn created_at(&self) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339(self.created_at.as_str()).unwrap()
    }

    pub fn updated_at(&self) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339(self.updated_at.as_str()).unwrap()
    }

    pub async fn owner(&self, context: &Context) -> User {
        context
            .user_data
            .as_ref()
            .unwrap()
            .user_by_id(self.owner_id)
            .await
    }
}

impl From<schema::courses::UserAnchor> for UserAnchor {
    fn from(x: schema::courses::UserAnchor) -> Self {
        Self {
            id: x.id,
            title: x.title,
            track_time: x.track_time as f64,
            position_top: x.position_top as f64,
            position_left: x.position_left as f64,
            page_id: x.page_id,
            track_id: x.track_id,
            created_at: x.created_at,
            updated_at: x.updated_at,
            owner_id: x.owner,
        }
    }
}
