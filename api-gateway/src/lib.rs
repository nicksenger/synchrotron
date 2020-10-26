use std::sync::Arc;

pub mod data;
pub mod entities;
pub mod errors;
pub mod graphql;

pub struct AppData {
  pub schema: Arc<graphql::schema::Schema>,
  pub user_channel: tonic::transport::Channel,
}
