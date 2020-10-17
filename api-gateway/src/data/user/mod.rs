use std::sync::Arc;

use crate::entities::User;

pub mod get_user_by_id;
use get_user_by_id::{get_loader, UserLoader};

#[derive(Clone)]
pub struct UserData {
    user_by_id: UserLoader,
}

impl UserData {
    pub fn new() -> Self {
        Self {
            user_by_id: get_loader(),
        }
    }

    pub async fn user_by_id(&self, id: i32) -> User {
        self.user_by_id.load(id).await
    }
}
