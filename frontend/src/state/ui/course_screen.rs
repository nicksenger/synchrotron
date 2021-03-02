use crate::messages::{application, ui, ui::course, Msg};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CursorMode {
    Default,
    Move,
    Delete,
    Add,
    Upgrade,
}

pub struct Model {
    pub loading: bool,
    pub relative_scroll: f32,
    pub mode: CursorMode,
    pub drag_x: i32,
    pub drag_y: i32,
}

impl Model {
    pub fn new() -> Self {
        Self {
            loading: false,
            relative_scroll: 0.0,
            mode: CursorMode::Default,
            drag_x: 0,
            drag_y: 0,
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
            Msg::Ui(ui::Msg::Course(course::Msg::ToggleMode(cursor_mode))) => {
                if &self.mode == cursor_mode {
                    self.mode = CursorMode::Default;
                } else {
                    self.mode = cursor_mode.clone();
                }
            }
            Msg::Ui(ui::Msg::Course(course::Msg::DragStart(payload))) => {
                self.drag_x = payload.x;
                self.drag_y = payload.y;
            }
            _ => {}
        }
    }
}
