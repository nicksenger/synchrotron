use crate::{
    entities::{Bookmark, DocumentBookmarks},
    errors::GatewayError,
};

mod bookmarks_by_id;
mod document_bookmarks;

use bookmarks_by_id::{get_loader, BookmarkLoader};

#[derive(Clone)]
pub struct BookmarkData {
    channel: tonic::transport::Channel,
    bookmarks_by_id: BookmarkLoader,
}

impl BookmarkData {
    pub fn new(channel: tonic::transport::Channel) -> Self {
        Self {
            bookmarks_by_id: get_loader(channel.clone()),
            channel,
        }
    }

    pub async fn bookmarks_by_id(&self, id: i32) -> Bookmark {
        self.bookmarks_by_id.load(id).await
    }

    pub async fn document_bookmarks(
        &self,
        data: DocumentBookmarks,
    ) -> Result<Vec<Bookmark>, GatewayError> {
        document_bookmarks::document_bookmarks(
            self.channel.clone(),
            data.document_id,
            data.limit,
            data.offset,
        )
        .await
    }
}
