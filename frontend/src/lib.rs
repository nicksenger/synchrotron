use iced_web::{Application, Command, Element};
use wasm_bindgen::prelude::*;

mod commands;
mod messages;
mod state;
mod view;

#[wasm_bindgen]
pub fn main() {
    Synchrotron::run(());
}

pub struct Synchrotron {
    state: state::Model,
}

impl Synchrotron {
    pub fn new() -> Self {
        Self {
            state: state::Model::new("".to_owned()),
        }
    }
}

impl Application for Synchrotron {
    type Executor = iced::executor::Default;
    type Message = messages::Msg;
    type Flags = ();

    fn title(&self) -> String {
        match &self.state.routing.route {
            state::Route::Login => "Synchrotron - Login".to_owned(),
            state::Route::Register => "Synchrotron - Register".to_owned(),
            state::Route::Courses => "Synchrotron - Courses".to_owned(),
            state::Route::Course(id, _) => format!(
                "Synchrotron - {}",
                self.state
                    .entities
                    .documents_by_id
                    .get(id)
                    .map(|c| c.title.clone())
                    .unwrap_or("Unknown Course".to_owned())
            ),
            _ => "Synchrotron - Not Found".to_owned(),
        }
    }

    fn new(_flags: ()) -> (Synchrotron, Command<Self::Message>) {
        (Synchrotron::new(), Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.state.update(&message);
        commands::get_command(&message, &self.state)
    }

    fn view(&mut self) -> Element<Self::Message> {
        view::View::new(&self.state).into()
    }
}
