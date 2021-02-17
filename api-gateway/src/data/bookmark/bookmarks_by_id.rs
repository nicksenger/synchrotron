use std::collections::HashMap;

use async_trait::async_trait;
use dataloader::{cached::Loader, BatchFn};
use schema::courses::{courses_client::CoursesClient, GetBookmarksByIDsRequest};
use tonic::transport::Channel;

use crate::{entities::Bookmark, errors::GatewayError};

async fn get_bookmark_by_id(
    map: &mut HashMap<i32, Bookmark>,
    ids: Vec<i32>,
    mut client: CoursesClient<Channel>,
) -> Result<(), GatewayError> {
    let request = tonic::Request::new(GetBookmarksByIDsRequest { ids });
    let response = client.get_bookmarks_by_ids(request).await?.into_inner();

    for b in response.bookmarks {
        map.insert(b.id, b.into());
    }

    Ok(())
}

pub struct BookmarkBatcher {
    channel: Channel,
}

impl BookmarkBatcher {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

#[async_trait]
impl BatchFn<i32, Bookmark> for BookmarkBatcher {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, Bookmark> {
        let client = CoursesClient::new(self.channel.clone());

        let mut anchor_map = HashMap::new();
        let _ = get_bookmark_by_id(&mut anchor_map, keys.to_vec(), client).await;
        anchor_map
    }
}

pub type BookmarkLoader = Loader<i32, Bookmark, BookmarkBatcher>;

pub fn get_loader(channel: Channel) -> BookmarkLoader {
    Loader::new(BookmarkBatcher::new(channel))
}
