use futures::future::ready;
use iced::Command;

use crate::{
    messages::{application, routing, Msg},
    state::Route,
};

pub fn get_command(msg: &routing::Msg) -> Command<Msg> {
    match msg {
        routing::Msg::Push(r) => {
            let new_url = String::from(r.clone());
            let _ = web_sys::window().and_then(|window| {
                window.history().ok().and_then(|history| {
                    history
                        .push_state_with_url(
                            &wasm_bindgen::JsValue::null(),
                            "",
                            Some(new_url.as_str()),
                        )
                        .ok()
                })
            });
            Command::perform(ready(r.clone()), |r| {
                Msg::Routing(routing::Msg::Navigate(r))
            })
        }
        routing::Msg::Replace(r) => {
            let new_url = String::from(r.clone());
            let _ = web_sys::window().and_then(|window| {
                window.history().ok().and_then(|history| {
                    history
                        .replace_state_with_url(
                            &wasm_bindgen::JsValue::null(),
                            "",
                            Some(new_url.as_str()),
                        )
                        .ok()
                })
            });

            Command::none()
        }
        routing::Msg::Navigate(r) => match r {
            Route::Courses => Command::perform(ready(()), |_| {
                Msg::Application(application::Msg::AllDocumentsRequest(
                    application::AllDocumentsRequestPayload {
                        limit: std::i32::MAX,
                        offset: 0,
                    },
                ))
            }),
            Route::Course(id, _anchor) => {
                let document_id = id.clone();
                Command::perform(ready(()), move |_| {
                    Msg::Application(application::Msg::DocumentRequest(
                        application::DocumentRequestPayload { document_id },
                    ))
                })
            }
            _ => Command::none(),
        },
        _ => Command::none(),
    }
}
