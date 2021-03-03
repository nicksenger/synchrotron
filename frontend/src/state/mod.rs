mod authentication;
pub mod entities;
mod routing;
pub mod ui;

pub use routing::Route;
use crate::messages::Msg;

pub struct Model {
    pub authentication: authentication::Model,
    pub entities: entities::Model,
    pub ui: ui::Model,
    pub routing: routing::Model,
}

impl Model {
    pub fn new(pathname: String) -> Self {
        Self {
            authentication: authentication::Model::default(),
            entities: entities::Model::default(),
            ui: ui::Model::new(),
            routing: routing::Model::new(pathname),
        }
    }

    pub fn update(&mut self, message: &Msg) {
        self.authentication.update(&message);
        self.entities.update(&message);
        self.routing.update(&message);
        self.ui.update(&message);
    }
}
