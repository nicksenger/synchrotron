use futures::future::ready;
use iced::Command;

use super::operations;
use crate::{
    messages::{
        application::{self, PageRequestPayload},
        Msg,
    },
    state::Model,
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
        _ => Command::none(),
    }
}
