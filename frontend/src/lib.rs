use std::cell::RefCell;
use std::rc::Rc;

use futures::future::ready;
use iced_futures::futures::channel::mpsc;
use iced_web::{Application, Command, Element, Subscription};
use wasm_bindgen::prelude::*;

mod commands;
mod effects;
mod messages;
mod state;
mod subscription;
mod view;

use messages::{routing, Msg};

#[wasm_bindgen]
pub fn main() {
    Synchrotron::run(());
}

pub struct Synchrotron {
    state: state::Model,
    message_sender: mpsc::UnboundedSender<Rc<Msg>>,
    message_receiver: Rc<RefCell<Option<mpsc::UnboundedReceiver<Rc<Msg>>>>>,
}

impl Synchrotron {
    pub fn new() -> Self {
        let pathname = web_sys::window()
            .and_then(|window| window.location().pathname().ok())
            .unwrap_or("".to_owned());

        let (message_sender, message_receiver) = mpsc::unbounded();

        Self {
            state: state::Model::new(pathname),
            message_sender,
            message_receiver: Rc::new(RefCell::new(Some(message_receiver))),
        }
    }
}

impl Application for Synchrotron {
    type Executor = iced::executor::Default;
    type Message = Msg;
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
        let synchrotron = Synchrotron::new();
        let route = synchrotron.state.routing.route.clone();
        (
            synchrotron,
            Command::perform(ready(routing::Msg::Navigate(route)), |msg| {
                Msg::Routing(msg)
            }),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.state.update(&message);
        self.message_sender.unbounded_send(Rc::new(message.clone()));
        commands::get_command(&message, &self.state)
    }

    fn view(&mut self) -> Element<Self::Message> {
        view::View::new(&self.state).into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::subscribe(self.message_receiver.borrow_mut().take())
    }
}
