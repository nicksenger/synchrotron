use std::collections::HashMap;

use async_trait::async_trait;
use dataloader::{cached::Loader, BatchFn};
use schema::courses::{courses_client::CoursesClient, GetPagesByIDsRequest};
use tonic::transport::Channel;

use crate::{entities::Page, errors::GatewayError};

async fn get_page_by_id(
    map: &mut HashMap<i32, Page>,
    ids: Vec<i32>,
    mut client: CoursesClient<Channel>,
) -> Result<(), GatewayError> {
    let request = tonic::Request::new(GetPagesByIDsRequest { ids });
    let response = client.get_pages_by_ids(request).await?.into_inner();

    for p in response.pages {
        map.insert(p.id, p.into());
    }

    Ok(())
}

pub struct PageBatcher {
    channel: Channel,
}

impl PageBatcher {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

#[async_trait]
impl BatchFn<i32, Page> for PageBatcher {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, Page> {
        let client = CoursesClient::new(self.channel.clone());

        let mut anchor_map = HashMap::new();
        let _ = get_page_by_id(&mut anchor_map, keys.to_vec(), client).await;
        anchor_map
    }
}

pub type PageLoader = Loader<i32, Page, PageBatcher>;

pub fn get_loader(channel: Channel) -> PageLoader {
    Loader::new(PageBatcher::new(channel))
}
