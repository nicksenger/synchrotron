use crate::messages::{application, ui, ui::course, Msg};

pub struct Model {
    pub loading: bool,
    pub relative_scroll: f32,
}

impl Model {
    pub fn new() -> Self {
        Self {
            loading: false,
            relative_scroll: 0.0,
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
            Msg::Ui(ui::Msg::Course(course::Msg::UpdateRelativeScroll(relative_scroll))) => {
                self.relative_scroll = *relative_scroll;
            }
            _ => {}
        }
    }
}
