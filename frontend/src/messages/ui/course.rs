use crate::state::ui::course_screen::CursorMode;

#[derive(Clone, Debug)]
pub enum Msg {
    UpdateRelativeScroll(f32),
    SelectBookmark(i32),
    SelectTrack(i32),
    SelectAnchor(i32),
    SelectUserAnchor(i32),
    ToggleMode(CursorMode),
    TogglePlayback,
}