use std::collections::{HashMap, HashSet};

use chrono::{DateTime, FixedOffset, TimeZone};

use crate::messages::{application, authentication, Msg};

#[derive(Clone, PartialEq, Debug)]
pub struct Document {
    pub id: i32,
    pub title: std::string::String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Page {
    pub id: i32,
    pub page_number: i32,
    pub image_path: std::string::String,
    pub aspect_ratio: f32,
    pub height: f32,
    pub document_id: i32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Track {
    pub id: i32,
    pub track_number: i32,
    pub title: std::string::String,
    pub audio_path: std::string::String,
    pub document_id: i32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Bookmark {
    pub id: i32,
    pub title: std::string::String,
    pub page_id: i32,
    pub document_id: i32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Anchor {
    pub id: i32,
    pub title: std::string::String,
    pub track_time: f32,
    pub position_top: f32,
    pub position_left: f32,
    pub page_id: i32,
    pub track_id: i32,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct UserAnchor {
    pub id: i32,
    pub title: std::string::String,
    pub track_time: f32,
    pub position_top: f32,
    pub position_left: f32,
    pub page_id: i32,
    pub track_id: i32,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub owner: i32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub username: std::string::String,
    pub role: i32,
}

#[derive(Default)]
pub struct Model {
    pub documents_by_id: HashMap<i32, Document>,
    pub bookmarks_by_id: HashMap<i32, Bookmark>,
    pub anchors_by_id: HashMap<i32, Anchor>,
    pub user_anchors_by_id: HashMap<i32, UserAnchor>,
    pub pages_by_id: HashMap<i32, Page>,
    pub tracks_by_id: HashMap<i32, Track>,
    pub users_by_id: HashMap<i32, User>,
    pub document_bookmarks: HashMap<i32, Vec<i32>>,
    pub document_tracks: HashMap<i32, Vec<i32>>,
    pub document_pages: HashMap<i32, Vec<i32>>,
    pub page_anchors: HashMap<i32, HashSet<i32>>,
    pub page_user_anchors: HashMap<i32, HashSet<i32>>,
}

impl Model {
    pub fn update(&mut self, message: &Msg) {
        match message {
            Msg::Authentication(authentication::Msg::LoginResponse(Ok(x))) => {
                self.users_by_id.insert(x.user.id, x.user.clone());
            }
            Msg::Authentication(authentication::Msg::RegisterResponse(Ok(x))) => {
                self.users_by_id.insert(x.user.id, x.user.clone());
            }
            Msg::Application(application::Msg::AllDocumentsResponse(Ok(x))) => {
                x.documents.clone().into_iter().for_each(|d| {
                    self.documents_by_id.insert(d.id, d.clone());
                });
            }
            Msg::Application(application::Msg::DocumentResponse(Ok(x))) => {
                self.documents_by_id
                    .insert(x.document.id, x.document.clone());

                let mut document_pages = x.pages.clone();
                document_pages.sort_unstable_by_key(|p| p.page_number);
                for p in document_pages {
                    self.document_pages
                        .entry(x.document.id)
                        .or_insert(vec![])
                        .push(p.id);
                    self.pages_by_id.insert(p.id, p);
                }

                self.document_tracks.insert(x.document.id, vec![]);
                let mut document_tracks = x.tracks.clone();
                document_tracks.sort_unstable_by_key(|t| t.track_number);
                for t in document_tracks {
                    self.document_tracks
                        .entry(x.document.id)
                        .or_insert(vec![])
                        .push(t.id);
                    self.tracks_by_id.insert(t.id, t);
                }

                self.document_bookmarks.insert(x.document.id, vec![]);
                let mut document_bookmarks = x.bookmarks.clone();
                document_bookmarks.sort_unstable_by_key(|b| {
                    self.pages_by_id.get(&b.page_id).unwrap().page_number
                });
                for b in document_bookmarks {
                    self.document_bookmarks
                        .entry(x.document.id)
                        .or_insert(vec![])
                        .push(b.id);
                    self.bookmarks_by_id.insert(b.id, b);
                }
            }
            Msg::Application(application::Msg::PageRequest(payload)) => {
                self.page_anchors.insert(payload.page_id, HashSet::new());
                self.page_user_anchors
                    .insert(payload.page_id, HashSet::new());
            }
            Msg::Application(application::Msg::PageResponse(Ok(x))) => {
                self.pages_by_id.insert(x.page.id, x.page.clone());
                for anchor in x.anchors.clone() {
                    self.page_anchors
                        .entry(x.page.id)
                        .or_insert(HashSet::new())
                        .insert(anchor.id);
                    self.anchors_by_id.insert(anchor.id, anchor);
                }
                for user_anchor in x.user_anchors.clone() {
                    self.page_user_anchors
                        .entry(x.page.id)
                        .or_insert(HashSet::new())
                        .insert(user_anchor.id);
                    self.user_anchors_by_id.insert(user_anchor.id, user_anchor);
                }
            }
            Msg::Application(application::Msg::CreateAnchorResponse(Ok(x))) => {
                self.page_anchors
                    .entry(x.anchor.page_id)
                    .or_insert(HashSet::new())
                    .insert(x.anchor.id);
                self.anchors_by_id.insert(x.anchor.id, x.anchor.clone());
            }
            Msg::Application(application::Msg::CreateUserAnchorRequest(payload)) => {
                self.page_user_anchors
                    .entry(payload.page_id)
                    .or_insert(HashSet::new())
                    .insert(std::i32::MAX);
                self.user_anchors_by_id.insert(
                    std::i32::MAX,
                    UserAnchor {
                        id: std::i32::MAX,
                        created_at: chrono::FixedOffset::west(5 * 10)
                            .ymd(2016, 11, 08)
                            .and_hms(0, 0, 0),
                        updated_at: chrono::FixedOffset::west(5 * 10)
                            .ymd(2016, 11, 08)
                            .and_hms(0, 0, 0),
                        position_top: payload.position_top,
                        position_left: payload.position_left,
                        owner: 0,
                        title: payload.title.clone(),
                        page_id: payload.page_id,
                        track_id: payload.track_id,
                        track_time: payload.track_time,
                    },
                );
            }
            Msg::Application(application::Msg::CreateUserAnchorResponse(resp)) => {
                self.page_user_anchors.values_mut().for_each(|s| {
                    s.remove(&std::i32::MAX);
                });

                self.user_anchors_by_id.remove(&std::i32::MAX);

                if let Ok(x) = resp {
                    self.page_user_anchors
                        .entry(x.user_anchor.page_id)
                        .or_insert(HashSet::new())
                        .insert(x.user_anchor.id);
                    self.user_anchors_by_id
                        .insert(x.user_anchor.id, x.user_anchor.clone());
                }
            }
            Msg::Application(application::Msg::DeleteAnchorRequest(x)) => {
                if let Some(set) = self
                    .page_anchors
                    .get_mut(&self.anchors_by_id.get(&x.anchor_id).unwrap().page_id)
                {
                    set.remove(&x.anchor_id);
                }
                self.anchors_by_id.remove(&x.anchor_id);
            }
            Msg::Application(application::Msg::DeleteUserAnchorRequest(x)) => {
                if let Some(set) = self
                    .page_user_anchors
                    .get_mut(&self.user_anchors_by_id.get(&x.anchor_id).unwrap().page_id)
                {
                    set.remove(&x.anchor_id);
                }
                self.user_anchors_by_id.remove(&x.anchor_id);
            }
            Msg::Application(application::Msg::JumpToAnchorResponse(Ok(payload))) => {
                self.anchors_by_id
                    .insert(payload.anchor.id, payload.anchor.clone());
                self.page_anchors
                    .entry(payload.anchor.page_id)
                    .or_insert(HashSet::new())
                    .insert(payload.anchor.id);
            }
            _ => {}
        }
    }
}
