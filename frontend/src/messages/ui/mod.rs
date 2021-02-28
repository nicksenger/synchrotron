pub mod login;
pub mod register;

#[derive(Clone, Debug)]
pub enum Msg {
    Login(login::Msg),
    Register(register::Msg),
}
