use crate::{entities::Document, errors::GatewayError};

mod all_documents;
mod get_document_by_id;

use get_document_by_id::{get_loader, DocumentLoader};

#[derive(Clone)]
pub struct DocumentData {
    channel: tonic::transport::Channel,
    documents_by_id: DocumentLoader,
}

impl DocumentData {
    pub fn new(channel: tonic::transport::Channel) -> Self {
        Self {
            documents_by_id: get_loader(channel.clone()),
            channel,
        }
    }

    pub async fn documents_by_id(&self, id: i32) -> Document {
        self.documents_by_id.load(id).await
    }

    pub async fn all_documents(
        &self,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Document>, GatewayError> {
        all_documents::all_documents(self.channel.clone(), limit, offset).await
    }
}
