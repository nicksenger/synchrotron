use crate::state::ui::course_screen::CursorMode;

#[derive(Clone, Debug)]
pub struct PageClickPayload {
    pub page_id: i32,
    pub position_top: f32,
    pub position_left: f32,
}

#[derive(Clone, Debug)]
pub struct DragStartPayload {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug)]
pub struct DragAnchorPayload {
    pub anchor_id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug)]
pub struct DragUserAnchorPayload {
    pub user_anchor_id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug)]
pub enum Msg {
    UpdateRelativeScroll(f32),
    SelectBookmark(i32),
    SelectTrack(i32),
    SelectAnchor(i32),
    DragStart(DragStartPayload),
    DragAnchor(DragAnchorPayload),
    SelectUserAnchor(i32),
    DragUserAnchor(DragUserAnchorPayload),
    ToggleMode(CursorMode),
    TogglePlayback,
    PageClick(PageClickPayload),
}
