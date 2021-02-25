use crate::state::Route;

#[derive(Debug)]
pub enum Msg {
    Navigate(Route),
    Push(Route),
}
