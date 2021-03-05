use futures::future::ready;
use iced::Command;
use wasm_bindgen::JsCast;

use super::operations;
use crate::{
    messages::{
        application::{self, JumpToAnchorRequestPayload, PageRequestPayload},
        Msg,
    },
    state::{Model, Route},
};

pub fn get_command(msg: &application::Msg, state: &Model) -> Command<Msg> {
    match msg {
        application::Msg::AllDocumentsRequest(payload) => Command::perform(
            operations::all_documents(payload.clone(), state.authentication.token.clone()),
            |x| Msg::Application(application::Msg::AllDocumentsResponse(x)),
        ),
        application::Msg::DocumentRequest(payload) => Command::perform(
            operations::document(payload.clone(), state.authentication.token.clone()),
            |x| Msg::Application(application::Msg::DocumentResponse(x)),
        ),
        application::Msg::PageRequest(payload) => Command::perform(
            operations::page(payload.clone(), state.authentication.token.clone()),
            |x| Msg::Application(application::Msg::PageResponse(x)),
        ),
        application::Msg::CreateAnchorRequest(payload) => Command::perform(
            operations::create_anchor(payload.clone(), state.authentication.token.clone()),
            |x| Msg::Application(application::Msg::CreateAnchorResponse(x)),
        ),
        application::Msg::CreateUserAnchorRequest(payload) => Command::perform(
            operations::create_user_anchor(payload.clone(), state.authentication.token.clone()),
            |x| Msg::Application(application::Msg::CreateUserAnchorResponse(x)),
        ),
        application::Msg::CreateUserAnchorResponse(Ok(payload)) => Command::perform(
            ready(application::Msg::PageRequest(PageRequestPayload {
                page_id: payload.user_anchor.page_id,
            })),
            Msg::Application,
        ),
        application::Msg::DeleteAnchorRequest(payload) => Command::perform(
            operations::delete_anchor(payload.clone(), state.authentication.token.clone()),
            |x| Msg::Application(application::Msg::DeleteAnchorResponse(x)),
        ),
        application::Msg::DeleteUserAnchorRequest(payload) => Command::perform(
            operations::delete_user_anchor(payload.clone(), state.authentication.token.clone()),
            |x| Msg::Application(application::Msg::DeleteUserAnchorResponse(x)),
        ),
        application::Msg::DocumentResponse(Ok(_)) => {
            if let Route::Course(_, Some(anchor_id)) = state.routing.route {
                Command::perform(
                    operations::jump_to_anchor(
                        JumpToAnchorRequestPayload { anchor_id },
                        state.authentication.token.clone(),
                    ),
                    |x| Msg::Application(application::Msg::JumpToAnchorResponse(x)),
                )
            } else {
                Command::none()
            }
        }
        application::Msg::JumpToAnchorResponse(Ok(payload)) => {
            let anchor_id = payload.anchor.id;
            Command::perform(
                operations::page(PageRequestPayload {
                    page_id: payload.anchor.page_id
                }, state.authentication.token.clone()),
                move |x| {
                    let window = web_sys::window().expect("no global `window` exists");
                    let document = window.document().expect("should have a document on window");

                    document
                        .get_element_by_id(format!("a-{}", anchor_id).as_str())
                        .and_then(|x| {
                            x.dyn_into::<web_sys::HtmlElement>().ok()
                        })
                        .and_then(|el| Some(el.scroll_into_view()));
                    
                    Msg::Application(application::Msg::PageResponse(x))
                }
            )
        }
        _ => Command::none(),
    }
}
