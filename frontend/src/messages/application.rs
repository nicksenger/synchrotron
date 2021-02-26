use super::ErrorPayload;
use crate::state::entities::{Anchor, Bookmark, Document, Page, Track, UserAnchor};

#[derive(Clone, Debug)]
pub struct AllDocumentsRequestPayload {
    pub limit: i32,
    pub offset: i32,
}

#[derive(Clone, Debug)]
pub struct AllDocumentsSuccessPayload {
    pub documents: Vec<Document>,
}

#[derive(Clone, Debug)]
pub struct DocumentRequestPayload {
    pub document_id: i32,
}

#[derive(Clone, Debug)]
pub struct DocumentSuccessPayload {
    pub document: Document,
    pub bookmarks: Vec<Bookmark>,
    pub pages: Vec<Page>,
    pub tracks: Vec<Track>,
}

#[derive(Clone, Debug)]
pub struct PageRequestPayload {
    pub page_id: i32,
}

#[derive(Clone, Debug)]
pub struct PageSuccessPayload {
    pub page: Page,
    pub anchors: Vec<Anchor>,
    pub user_anchors: Vec<UserAnchor>,
}
#[derive(Clone, Debug)]
pub struct CreateAnchorRequestPayload {
    pub title: String,
    pub track_time: f32,
    pub position_top: f32,
    pub position_left: f32,
    pub page_id: i32,
    pub track_id: i32,
}

#[derive(Clone, Debug)]
pub struct CreateAnchorSuccessPayload {
    pub anchor: Anchor,
}

#[derive(Clone, Debug)]
pub struct CreateUserAnchorSuccessPayload {
    pub user_anchor: UserAnchor,
}

#[derive(Clone, Debug)]
pub struct DeleteAnchorRequestPayload {
    pub anchor_id: i32,
}

#[derive(Clone, Debug)]
pub struct DeleteAnchorSuccessPayload {
    pub success: bool,
}

#[derive(Clone, Debug)]
pub enum Msg {
    AllDocumentsRequest(AllDocumentsRequestPayload),
    AllDocumentsResponse(Result<AllDocumentsSuccessPayload, ErrorPayload>),
    DocumentRequest(DocumentRequestPayload),
    DocumentResponse(Result<DocumentSuccessPayload, ErrorPayload>),
    PageRequest(PageRequestPayload),
    PageResponse(Result<PageSuccessPayload, ErrorPayload>),
    CreateAnchorRequest(CreateAnchorRequestPayload),
    CreateAnchorResponse(Result<CreateAnchorSuccessPayload, ErrorPayload>),
    DeleteAnchorRequest(DeleteAnchorRequestPayload),
    DeleteAnchorResponse(Result<DeleteAnchorSuccessPayload, ErrorPayload>),
    CreateUserAnchorRequest(CreateAnchorRequestPayload),
    CreateUserAnchorResponse(Result<CreateUserAnchorSuccessPayload, ErrorPayload>),
    DeleteUserAnchorRequest(DeleteAnchorRequestPayload),
    DeleteUserAnchorResponse(Result<DeleteAnchorSuccessPayload, ErrorPayload>),
}
