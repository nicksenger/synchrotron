use std::collections::HashMap;

use async_trait::async_trait;
use dataloader::{cached::Loader, BatchFn};
use schema::courses::{courses_client::CoursesClient, GetDocumentsByIDsRequest};
use tonic::transport::Channel;

use crate::{entities::Document, errors::GatewayError};

async fn get_document_by_id(
    map: &mut HashMap<i32, Document>,
    ids: Vec<i32>,
    mut client: CoursesClient<Channel>,
) -> Result<(), GatewayError> {
    let request = tonic::Request::new(GetDocumentsByIDsRequest { ids });
    let response = client.get_documents_by_ids(request).await?.into_inner();

    for d in response.documents {
        map.insert(d.id, d.into());
    }

    Ok(())
}

pub struct DocumentBatcher {
    channel: Channel,
}

impl DocumentBatcher {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

#[async_trait]
impl BatchFn<i32, Document> for DocumentBatcher {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, Document> {
        let client = CoursesClient::new(self.channel.clone());

        let mut anchor_map = HashMap::new();
        let _ = get_document_by_id(&mut anchor_map, keys.to_vec(), client).await;
        anchor_map
    }
}

pub type DocumentLoader = Loader<i32, Document, DocumentBatcher>;

pub fn get_loader(channel: Channel) -> DocumentLoader {
    Loader::new(DocumentBatcher::new(channel))
}
