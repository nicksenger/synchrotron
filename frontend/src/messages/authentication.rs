use super::ErrorPayload;
use crate::state::entities::User;

#[derive(Clone, Debug)]
pub struct LoginRequestPayload {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct LoginSuccessPayload {
    pub token: String,
    pub user: User,
}

#[derive(Clone, Debug)]
pub struct RegisterRequestPayload {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct RegisterSuccessPayload {
    pub user: User,
}

#[derive(Clone, Debug)]
pub enum Msg {
    LoginRequest(LoginRequestPayload),
    LoginResponse(Result<LoginSuccessPayload, ErrorPayload>),
    RegisterRequest(RegisterRequestPayload),
    RegisterResponse(Result<RegisterSuccessPayload, ErrorPayload>),
    Logout,
}
