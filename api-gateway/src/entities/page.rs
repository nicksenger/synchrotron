use std::convert::From;

use super::{Anchor, Document, UserAnchor};
use crate::graphql::schema::Context;

#[derive(Debug, Clone)]
/// Page of a document
pub struct Page {
    // ID of the page
    pub id: i32,
    // Number of the page
    pub page_number: i32,
    // Path to the image for the page
    pub image_path: String,
    // Aspect ratio of the page
    pub aspect_ratio: f64,
    // Height of the page
    pub height: f64,
    // Document ID of the page
    pub document_id: i32,
}

#[juniper::graphql_object(Context = Context)]
impl Page {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn page_number(&self) -> i32 {
        self.page_number
    }

    pub fn image_path(&self) -> &str {
        self.image_path.as_str()
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub async fn document(&self, context: &Context) -> Document {
        context
            .document_data
            .as_ref()
            .unwrap()
            .documents_by_id(self.document_id)
            .await
    }

    pub async fn anchors(&self, context: &Context) -> Vec<Anchor> {
        context
            .anchor_data
            .as_ref()
            .unwrap()
            .page_anchors(self.id)
            .await
            .unwrap()
    }

    pub async fn user_anchors(&self, context: &Context) -> Vec<UserAnchor> {
        context
            .user_anchor_data
            .as_ref()
            .unwrap()
            .page_user_anchors(self.id)
            .await
            .unwrap()
    }
}

impl From<schema::courses::Page> for Page {
    fn from(x: schema::courses::Page) -> Page {
        Page {
            id: x.id,
            page_number: x.page_number,
            image_path: x.image_path,
            aspect_ratio: x.aspect_ratio as f64,
            height: x.height as f64,
            document_id: x.document_id,
        }
    }
}
