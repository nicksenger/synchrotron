use futures::future::ready;
use iced::Command;

use super::operations;
use crate::{
    messages::{authentication, routing, Msg},
    state::Route,
};

pub fn get_command(msg: &authentication::Msg) -> Command<Msg> {
    match msg {
        authentication::Msg::LoginRequest(payload) => {
            Command::perform(operations::login(payload.clone()), |x| {
                Msg::Authentication(authentication::Msg::LoginResponse(x))
            })
        }
        authentication::Msg::RegisterRequest(payload) => {
            Command::perform(operations::register(payload.clone()), |x| {
                Msg::Authentication(authentication::Msg::RegisterResponse(x))
            })
        }
        authentication::Msg::LoginResponse(Ok(_)) => Command::perform(ready(Route::Courses), |r| {
            Msg::Routing(routing::Msg::Push(r))
        }),
        authentication::Msg::RegisterResponse(Ok(_)) => {
            Command::perform(ready(Route::Login), |r| Msg::Routing(routing::Msg::Push(r)))
        }
        _ => Command::none(),
    }
}
