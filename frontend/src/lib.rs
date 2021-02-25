use iced::{Application, Command, Settings, Element, Text};

mod commands;
mod messages;
mod state;

pub fn main() -> iced::Result {
    Synchrotron::run(Settings::default())
}

struct Synchrotron {
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
            _ => "Synchrotron - Not Found".to_owned(),
        }
    }

    fn new(_flags: ()) -> (Synchrotron, Command<Self::Message>) {
        (Synchrotron::new(), Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.state.update(&message);
        commands::get_command(&message)
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("test").into()
    }
}
