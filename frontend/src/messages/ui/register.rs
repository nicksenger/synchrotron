#[derive(Clone, Debug)]
pub enum Msg {
    UsernameInputChanged(String),
    PasswordInputChanged(String),
}
