pub mod login;
pub mod register;
pub mod course;

#[derive(Clone, Debug)]
pub enum Msg {
    Login(login::Msg),
    Register(register::Msg),
    Course(course::Msg),
}
