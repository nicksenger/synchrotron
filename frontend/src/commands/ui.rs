use futures::future::ready;
use iced::Command;
use wasm_bindgen::JsCast;

use crate::{
    messages::{
        application::{
            self, CreateAnchorRequestPayload, DeleteAnchorRequestPayload, PageRequestPayload,
        },
        routing, ui, Msg,
    },
    state::{ui::course_screen::CursorMode, Model, Route},
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

            el.set_src(format!("https://synchrotron.nsenger.com/{}", track.audio_path).as_str());
            el.set_current_time(0.0);
            el.play();

            Command::none()
        }
        ui::Msg::Course(ui::course::Msg::SelectAnchor(anchor_id)) => {
            match state.ui.course_screen.mode {
                CursorMode::Default => {
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

                    if let Route::Course(document_id, _) = state.routing.route {
                        return Command::perform(
                            ready(routing::Msg::Replace(Route::Course(document_id, Some(anchor.id)))),
                            Msg::Routing,
                        );
                    }

                    Command::none()
                }
                CursorMode::Delete => Command::perform(
                    ready(application::Msg::DeleteAnchorRequest(
                        DeleteAnchorRequestPayload {
                            anchor_id: *anchor_id,
                        },
                    )),
                    Msg::Application,
                ),
                _ => Command::none(),
            }
        }
        ui::Msg::Course(ui::course::Msg::SelectUserAnchor(user_anchor_id)) => {
            match state.ui.course_screen.mode {
                CursorMode::Default => {
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
                CursorMode::Delete => Command::perform(
                    ready(application::Msg::DeleteUserAnchorRequest(
                        DeleteAnchorRequestPayload {
                            anchor_id: *user_anchor_id,
                        },
                    )),
                    Msg::Application,
                ),
                CursorMode::Upgrade => {
                    if state
                        .authentication
                        .active_user
                        .map(|u| state.entities.users_by_id.get(&u).unwrap().role)
                        .unwrap_or(0)
                        > 0
                    {
                        let existing_anchor = state
                            .entities
                            .user_anchors_by_id
                            .get(user_anchor_id)
                            .unwrap();
                        return Command::batch(vec![
                            Command::perform(
                                ready(application::Msg::DeleteUserAnchorRequest(
                                    DeleteAnchorRequestPayload {
                                        anchor_id: *user_anchor_id,
                                    },
                                )),
                                Msg::Application,
                            ),
                            Command::perform(
                                ready(application::Msg::CreateAnchorRequest(
                                    CreateAnchorRequestPayload {
                                        title: existing_anchor.title.clone(),
                                        track_time: existing_anchor.track_time.clone(),
                                        position_top: existing_anchor.position_top.clone(),
                                        position_left: existing_anchor.position_left.clone(),
                                        page_id: existing_anchor.page_id,
                                        track_id: existing_anchor.track_id,
                                    },
                                )),
                                Msg::Application,
                            ),
                        ]);
                    }
                    Command::none()
                }
                _ => Command::none(),
            }
        }
        ui::Msg::Course(ui::course::Msg::DragAnchor(payload)) => {
            let window = web_sys::window().expect("no global `window` exists");
            let existing_anchor = state
                .entities
                .anchors_by_id
                .get(&payload.anchor_id)
                .unwrap();
            let inner_width: f32 = window.inner_width().unwrap().as_f64().unwrap() as f32;
            let page_height = state
                .entities
                .pages_by_id
                .get(&existing_anchor.page_id)
                .unwrap()
                .aspect_ratio
                * inner_width;
            Command::batch(vec![
                Command::perform(
                    ready(application::Msg::DeleteAnchorRequest(
                        DeleteAnchorRequestPayload {
                            anchor_id: payload.anchor_id,
                        },
                    )),
                    Msg::Application,
                ),
                Command::perform(
                    ready(application::Msg::CreateAnchorRequest(
                        CreateAnchorRequestPayload {
                            title: existing_anchor.title.clone(),
                            track_time: existing_anchor.track_time.clone(),
                            position_top: existing_anchor.position_top.clone()
                                + (((payload.y - state.ui.course_screen.drag_y) as f32
                                    / page_height)
                                    * 100.0),
                            position_left: existing_anchor.position_left.clone()
                                + (((payload.x - state.ui.course_screen.drag_x) as f32
                                    / inner_width)
                                    * 100.0),
                            page_id: existing_anchor.page_id,
                            track_id: existing_anchor.track_id,
                        },
                    )),
                    Msg::Application,
                ),
            ])
        }
        ui::Msg::Course(ui::course::Msg::DragUserAnchor(payload)) => {
            let window = web_sys::window().expect("no global `window` exists");
            let existing_anchor = state
                .entities
                .user_anchors_by_id
                .get(&payload.user_anchor_id)
                .unwrap();
            let inner_width: f32 = window.inner_width().unwrap().as_f64().unwrap() as f32;
            let page_height = state
                .entities
                .pages_by_id
                .get(&existing_anchor.page_id)
                .unwrap()
                .aspect_ratio
                * inner_width;
            Command::batch(vec![
                Command::perform(
                    ready(application::Msg::DeleteUserAnchorRequest(
                        DeleteAnchorRequestPayload {
                            anchor_id: payload.user_anchor_id,
                        },
                    )),
                    Msg::Application,
                ),
                Command::perform(
                    ready(application::Msg::CreateUserAnchorRequest(
                        CreateAnchorRequestPayload {
                            title: existing_anchor.title.clone(),
                            track_time: existing_anchor.track_time.clone(),
                            position_top: existing_anchor.position_top.clone()
                                + (((payload.y - state.ui.course_screen.drag_y) as f32
                                    / page_height)
                                    * 100.0),
                            position_left: existing_anchor.position_left.clone()
                                + (((payload.x - state.ui.course_screen.drag_x) as f32
                                    / inner_width)
                                    * 100.0),
                            page_id: existing_anchor.page_id,
                            track_id: existing_anchor.track_id,
                        },
                    )),
                    Msg::Application,
                ),
            ])
        }
        ui::Msg::Course(ui::course::Msg::SelectBookmark(bookmark_id)) => {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");

            let bookmark = state.entities.bookmarks_by_id.get(bookmark_id).unwrap();
            let el = document
                .get_element_by_id(format!("p-{}", bookmark.page_id).as_str())
                .unwrap()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap();

            el.scroll_into_view();

            Command::none()
        }
        ui::Msg::Course(ui::course::Msg::TogglePlayback) => {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
            let el = document
                .get_element_by_id("audio")
                .unwrap()
                .dyn_into::<web_sys::HtmlAudioElement>()
                .unwrap();

            if el.paused() {
                el.play();
            } else {
                el.pause();
            }

            Command::none()
        }
        ui::Msg::Course(ui::course::Msg::PageClick(payload)) => {
            if state.ui.course_screen.mode == CursorMode::Add {
                let window = web_sys::window().expect("no global `window` exists");
                let document = window.document().expect("should have a document on window");
                let el = document
                    .get_element_by_id("audio")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlAudioElement>()
                    .unwrap();

                let current_src = el.current_src();
                if current_src != "" {
                    return Command::perform(
                        ready(application::Msg::CreateUserAnchorRequest(
                            CreateAnchorRequestPayload {
                                title: "".to_owned(),
                                track_time: el.current_time() as f32,
                                position_top: payload.position_top,
                                position_left: payload.position_left,
                                page_id: payload.page_id,
                                track_id: state
                                    .entities
                                    .tracks_by_id
                                    .values()
                                    .find(|&t| current_src.find(&t.audio_path).is_some())
                                    .unwrap()
                                    .id,
                            },
                        )),
                        Msg::Application,
                    );
                }
            }
            Command::none()
        }
        _ => Command::none(),
    }
}
