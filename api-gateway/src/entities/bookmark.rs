use std::convert::From;

use super::Document;
use crate::graphql::schema::Context;

#[derive(Debug, Clone)]
/// Bookmark for a document
pub struct Bookmark {
    // ID of the bookmark
    pub id: i32,
    // Title of the bookmark
    pub title: String,
    // Page id for the bookmark
    pub page_id: i32,
    // Document id for the bookmark
    pub document_id: i32,
}

#[derive(juniper::GraphQLInputObject, Debug, Clone)]
// Retrieving bookmarks for a document
pub struct DocumentBookmarks {
    // Document to query for bookmarks
    pub document_id: i32,
    // Limit for the query
    pub limit: i32,
    // Offset for the query
    pub offset: i32,
}

#[juniper::graphql_object(Context = Context)]
impl Bookmark {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn page(&self, context: &Context) -> i32 {
        self.page_id
    }

    pub async fn document(&self, context: &Context) -> Document {
        context.document_data.as_ref().unwrap().documents_by_id(self.document_id).await
    }
}

impl From<schema::courses::Bookmark> for Bookmark {
    fn from(x: schema::courses::Bookmark) -> Bookmark {
        Bookmark {
            id: x.id,
            title: x.title,
            page_id: x.page_id,
            document_id: x.document_id,
        }
    }
}
