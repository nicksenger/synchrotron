pub mod application;
pub mod authentication;
pub mod routing;

pub use authentication::*;
pub use routing::*;

#[derive(Debug)]
pub struct ErrorPayload {
    pub content: String,
}

#[derive(Debug)]
pub enum Msg {
    Application(application::Msg),
    Authentication(authentication::Msg),
    Routing(routing::Msg),
}
