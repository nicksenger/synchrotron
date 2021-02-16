use juniper::EmptySubscription;

use super::{mutation::Mutation, query::Query};
use schema::shared::User;
use crate::data::UserData;

#[derive(Clone)]
pub struct Context {
    pub user: Option<User>,
    pub user_data: Option<UserData>,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new(user: Option<User>, user_data: Option<UserData>) -> Self {
        Self { user, user_data }
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, juniper::EmptySubscription::new())
}
