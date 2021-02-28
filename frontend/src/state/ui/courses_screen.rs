use std::collections::HashMap;

use iced::button;

use crate::messages::{application, ui, Msg};

pub struct Model {
    pub loading: bool,
    pub btn_states: HashMap<i32, button::State>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            loading: false,
            btn_states: HashMap::new(),
        }
    }

    pub fn update(&mut self, message: &Msg) {
        match message {
            Msg::Application(application::Msg::AllDocumentsRequest(_)) => {
                self.loading = true;
            }
            Msg::Application(application::Msg::AllDocumentsResponse(resp)) => {
                self.loading = false;

                if let Ok(result) = resp {
                    for d in &result.documents {
                        self.btn_states
                            .insert(d.id, button::State::default());
                    }
                }
            }
            _ => {}
        }
    }
}
