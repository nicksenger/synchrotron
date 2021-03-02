use futures::future::ready;
use iced::Command;
use wasm_bindgen::JsCast;

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
        ui::Msg::Course(ui::course::Msg::SelectTrack(track_id)) => {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");

            let track = state.entities.tracks_by_id.get(track_id).unwrap();
            let el = document
                .get_element_by_id("audio")
                .unwrap()
                .dyn_into::<web_sys::HtmlAudioElement>()
                .unwrap();

            el.set_src(
                format!(
                    "https://synchrotron.nsenger.com/{}",
                    track.audio_path
                )
                .as_str(),
            );
            el.set_current_time(0.0);
            el.play();

            Command::none()
        }
        ui::Msg::Course(ui::course::Msg::SelectAnchor(anchor_id)) => {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");

            let anchor = state.entities.anchors_by_id.get(anchor_id).unwrap();
            let el = document
                .get_element_by_id("audio")
                .unwrap()
                .dyn_into::<web_sys::HtmlAudioElement>()
                .unwrap();

            el.set_src(
                format!(
                    "https://synchrotron.nsenger.com/{}",
                    state
                        .entities
                        .tracks_by_id
                        .get(&anchor.track_id)
                        .unwrap()
                        .audio_path
                )
                .as_str(),
            );
            el.set_current_time(anchor.track_time as f64);
            el.play();

            Command::none()
        }
        ui::Msg::Course(ui::course::Msg::SelectUserAnchor(user_anchor_id)) => {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");

            let anchor = state
                .entities
                .user_anchors_by_id
                .get(user_anchor_id)
                .unwrap();
            let el = document
                .get_element_by_id("audio")
                .unwrap()
                .dyn_into::<web_sys::HtmlAudioElement>()
                .unwrap();

            el.set_src(
                format!(
                    "https://synchrotron.nsenger.com/{}",
                    state
                        .entities
                        .tracks_by_id
                        .get(&anchor.track_id)
                        .unwrap()
                        .audio_path
                )
                .as_str(),
            );
            el.set_current_time(anchor.track_time as f64);
            el.play();

            Command::none()
        }
        ui::Msg::Course(ui::course::Msg::SelectBookmark(bookmark_id)) => {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");

            let bookmark = state.entities.bookmarks_by_id.get(bookmark_id).unwrap();
            let el = document
                .get_element_by_id("page-container")
                .unwrap()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap();

            el.set_scroll_top(
                state
                    .entities
                    .pages_by_id
                    .get(&bookmark.page_id)
                    .unwrap()
                    .height as i32
                    * el.client_width(),
            );

            Command::none()
        }
        _ => Command::none(),
    }
}
