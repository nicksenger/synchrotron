use iced::Command;

use super::operations;
use crate::messages::{application, Msg};

pub fn get_command(msg: &application::Msg) -> Command<Msg> {
    match msg {
        application::Msg::AllDocumentsRequest(payload) => {
            Command::perform(operations::all_documents(payload.clone()), |x| {
                Msg::Application(application::Msg::AllDocumentsResponse(x))
            })
        }
        application::Msg::DocumentRequest(payload) => {
            Command::perform(operations::document(payload.clone()), |x| {
                Msg::Application(application::Msg::DocumentResponse(x))
            })
        }
        application::Msg::PageRequest(payload) => {
            Command::perform(operations::page(payload.clone()), |x| {
                Msg::Application(application::Msg::PageResponse(x))
            })
        }
        application::Msg::CreateAnchorRequest(payload) => {
            Command::perform(operations::create_anchor(payload.clone()), |x| {
                Msg::Application(application::Msg::CreateAnchorResponse(x))
            })
        }
        application::Msg::CreateUserAnchorRequest(payload) => {
            Command::perform(operations::create_user_anchor(payload.clone()), |x| {
                Msg::Application(application::Msg::CreateUserAnchorResponse(x))
            })
        }
        application::Msg::DeleteAnchorRequest(payload) => {
            Command::perform(operations::delete_anchor(payload.clone()), |x| {
                Msg::Application(application::Msg::DeleteAnchorResponse(x))
            })
        }
        application::Msg::DeleteUserAnchorRequest(payload) => {
            Command::perform(operations::delete_user_anchor(payload.clone()), |x| {
                Msg::Application(application::Msg::DeleteAnchorResponse(x))
            })
        }
        _ => Command::none(),
    }
}
