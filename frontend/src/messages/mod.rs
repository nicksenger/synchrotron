pub mod application;
pub mod authentication;
pub mod routing;
pub mod ui;

pub use authentication::*;
pub use routing::*;

#[derive(Clone, Debug)]
pub struct ErrorPayload {
    pub content: String,
}

#[derive(Clone, Debug)]
pub enum Msg {
    Application(application::Msg),
    Authentication(authentication::Msg),
    Routing(routing::Msg),
    Ui(ui::Msg),
}
