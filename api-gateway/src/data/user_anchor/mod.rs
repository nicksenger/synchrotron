use crate::{
    entities::{
        CreateUserAnchor, DeleteUserAnchor, DeleteUserAnchorResponse, PageUserAnchors, UserAnchor,
    },
    errors::GatewayError,
};
use schema::shared::User;

mod create_user_anchor;
mod delete_user_anchor;
mod page_user_anchors;
mod user_anchors_by_id;

use user_anchors_by_id::{get_loader, UserAnchorLoader};

#[derive(Clone)]
pub struct UserAnchorData {
    channel: tonic::transport::Channel,
    user_anchors_by_id: UserAnchorLoader,
}

impl UserAnchorData {
    pub fn new(channel: tonic::transport::Channel) -> Self {
        Self {
            user_anchors_by_id: get_loader(channel.clone()),
            channel,
        }
    }

    pub async fn user_anchors_by_id(&self, id: i32) -> UserAnchor {
        self.user_anchors_by_id.load(id).await
    }

    pub async fn page_user_anchors(
        &self,
        data: PageUserAnchors,
    ) -> Result<Vec<UserAnchor>, GatewayError> {
        page_user_anchors::page_user_anchors(self.channel.clone(), data.page_id).await
    }

    pub async fn create_user_anchor(
        &self,
        user: Option<User>,
        data: CreateUserAnchor,
    ) -> Result<UserAnchor, GatewayError> {
        let response =
            create_user_anchor::create_user_anchor(user, data, self.channel.clone()).await?;
        Ok(response.user_anchor.unwrap().into())
    }

    pub async fn delete_user_anchor(
        &self,
        user: Option<User>,
        data: DeleteUserAnchor,
    ) -> Result<DeleteUserAnchorResponse, GatewayError> {
        let response =
            delete_user_anchor::delete_user_anchor(user, data.anchor_id, self.channel.clone())
                .await?;
        Ok(DeleteUserAnchorResponse {
            success: response.success,
        })
    }
}
