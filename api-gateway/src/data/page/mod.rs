use crate::{
  entities::{Page, DocumentPages},
  errors::GatewayError,
};

mod pages_by_id;
mod document_pages;

use pages_by_id::{get_loader, PageLoader};

#[derive(Clone)]
pub struct PageData {
  channel: tonic::transport::Channel,
  pages_by_id: PageLoader,
}

impl PageData {
  pub fn new(channel: tonic::transport::Channel) -> Self {
      Self {
          pages_by_id: get_loader(channel.clone()),
          channel,
      }
  }

  pub async fn pages_by_id(&self, id: i32) -> Page {
      self.pages_by_id.load(id).await
  }

  pub async fn document_pages(
      &self,
      data: DocumentPages,
  ) -> Result<Vec<Page>, GatewayError> {
      document_pages::document_pages(
          self.channel.clone(),
          data.document_id,
          data.limit,
          data.offset,
      )
      .await
  }
}
