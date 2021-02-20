use schema::shared::User;

use crate::{
    entities::{Bookmark, DeleteBookmarkResponse},
    errors::GatewayError,
};

mod bookmarks_by_id;
mod create_bookmark;
mod delete_bookmark;
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
        document_id: i32,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Bookmark>, GatewayError> {
        document_bookmarks::document_bookmarks(self.channel.clone(), document_id, limit, offset)
            .await
    }

    pub async fn create_bookmark(
        &self,
        title: String,
        page_id: i32,
        document_id: i32,
        acitve_user: Option<User>,
    ) -> Result<Bookmark, GatewayError> {
        create_bookmark::create_bookmark(
            self.channel.clone(),
            title,
            page_id,
            document_id,
            acitve_user,
        )
        .await
    }

    pub async fn delete_bookmark(
        &self,
        bookmark_id: i32,
        acitve_user: Option<User>,
    ) -> Result<DeleteBookmarkResponse, GatewayError> {
        delete_bookmark::delete_bookmark(self.channel.clone(), bookmark_id, acitve_user).await
    }
}
