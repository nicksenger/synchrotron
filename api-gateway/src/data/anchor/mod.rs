use crate::{
    entities::{Anchor, CreateAnchor, DeleteAnchorResponse},
    errors::GatewayError,
};
use schema::shared::User;

mod anchors_by_id;
mod anchors_by_page_id;
mod create_anchor;
mod delete_anchor;

use anchors_by_id::{get_loader, AnchorLoader};
use anchors_by_page_id::{get_page_loader, PageAnchorLoader};

#[derive(Clone)]
pub struct AnchorData {
    channel: tonic::transport::Channel,
    anchors_by_id: AnchorLoader,
    anchors_by_page_id: PageAnchorLoader,
}

impl AnchorData {
    pub fn new(channel: tonic::transport::Channel) -> Self {
        Self {
            anchors_by_id: get_loader(channel.clone()),
            anchors_by_page_id: get_page_loader(channel.clone()),
            channel,
        }
    }

    pub async fn anchors_by_id(&self, id: i32) -> Anchor {
        self.anchors_by_id.load(id).await
    }

    pub async fn page_anchors(&self, page_id: i32) -> Vec<Anchor> {
        self.anchors_by_page_id.load(page_id).await
    }

    pub async fn create_anchor(
        &self,
        user: Option<User>,
        data: CreateAnchor,
    ) -> Result<Anchor, GatewayError> {
        let response = create_anchor::create_anchor(user, data, self.channel.clone()).await?;
        Ok(response.anchor.unwrap().into())
    }

    pub async fn delete_anchor(
        &self,
        user: Option<User>,
        anchor_id: i32,
    ) -> Result<DeleteAnchorResponse, GatewayError> {
        let response = delete_anchor::delete_anchor(user, anchor_id, self.channel.clone()).await?;
        Ok(DeleteAnchorResponse {
            success: response.success,
        })
    }
}
