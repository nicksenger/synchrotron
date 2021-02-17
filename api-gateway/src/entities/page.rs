use std::convert::From;

use super::Document;
use crate::graphql::schema::Context;

#[derive(Debug, Clone)]
/// Page of a document
pub struct Page {
    // ID of the page
    pub id: i32,
    // Number of the page
    pub page_number: i32,
    // Aspect ratio of the page
    pub aspect_ratio: f64,
    // Height of the page
    pub height: f64,
    // Document ID of the page
    pub document_id: i32,
}

#[derive(juniper::GraphQLInputObject, Debug, Clone)]
// Retrieving pages for a document
pub struct DocumentPages {
    // Document to query for pages
    pub document_id: i32,
    // Limit for the query
    pub limit: i32,
    // Offset for the query
    pub offset: i32,
}

#[juniper::graphql_object(Context = Context)]
impl Page {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn page_number(&self) -> i32 {
        self.page_number
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
}

impl From<schema::courses::Page> for Page {
    fn from(x: schema::courses::Page) -> Page {
        Page {
            id: x.id,
            page_number: x.page_number,
            aspect_ratio: x.aspect_ratio as f64,
            height: x.height as f64,
            document_id: x.document_id,
        }
    }
}
