pub mod login;

#[derive(Clone, Debug)]
pub enum Msg {
    Login(login::Msg),
}
