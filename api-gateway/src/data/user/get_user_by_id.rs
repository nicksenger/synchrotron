use std::{collections::HashMap};

use async_trait::async_trait;
use dataloader::{cached::Loader, BatchFn};

use crate::entities::User;

async fn get_anchor_by_ids(map: &mut HashMap<i32, User>, ids: Vec<i32>) {
    
}

pub struct UserBatcher {}

impl UserBatcher {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl BatchFn<i32, User> for UserBatcher {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, User> {
        let mut anchor_map = HashMap::new();
        get_anchor_by_ids(&mut anchor_map, keys.to_vec()).await;
        anchor_map
    }
}

pub type UserLoader = Loader<i32, User, UserBatcher>;

pub fn get_loader() -> UserLoader {
    Loader::new(UserBatcher::new())
}
