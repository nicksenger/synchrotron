use crate::state::Route;

#[derive(Clone, Debug)]
pub enum Msg {
    Navigate(Route),
    Push(Route),
    Replace(Route),
}
