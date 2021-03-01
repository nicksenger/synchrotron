use futures::future::ready;
use iced::Command;

use crate::{
    messages::{
        application::{self, PageRequestPayload},
        ui, Msg,
    },
    state::{Model, Route},
};

pub fn get_command(msg: &ui::Msg, state: &Model) -> Command<Msg> {
    match msg {
        ui::Msg::Course(ui::course::Msg::UpdateRelativeScroll(relative_scroll)) => {
            if !state.ui.course_screen.loading {
                if let Route::Course(course_id, _) = state.routing.route {
                    let (mut active_page, mut min_distance) = (0, std::f32::MAX);
                    for page_id in state.entities.document_pages.get(&course_id).unwrap() {
                        let distance = (relative_scroll
                            - state.entities.pages_by_id.get(page_id).unwrap().height)
                            .abs();
                        if distance < min_distance {
                            active_page = *page_id;
                            min_distance = distance;
                        }
                    }

                    if state.entities.page_anchors.get(&active_page).is_none() {
                        return Command::perform(ready(active_page), |page_id| {
                            Msg::Application(application::Msg::PageRequest(PageRequestPayload {
                                page_id,
                            }))
                        });
                    }
                }
            }
            Command::none()
        }
        _ => Command::none(),
    }
}
