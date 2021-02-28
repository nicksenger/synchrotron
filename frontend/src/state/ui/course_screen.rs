use crate::messages::{Msg, application};

pub struct Model {
    pub loading: bool,
}

impl Model {
    pub fn new() -> Self {
        Self {
            loading: false,
        }
    }

    pub fn update(&mut self, message: &Msg) {
        match message {
            Msg::Application(application::Msg::DocumentRequest(_)) => {
                self.loading = true;
            }
            Msg::Application(application::Msg::DocumentResponse(resp)) => {
                self.loading = false;
            }
            _ => {}
        }
    }
}
