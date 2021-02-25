use super::ErrorPayload;

#[derive(Debug)]
pub struct LoginRequestPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct LoginSuccessPayload {
    pub token: String,
    pub user_id: i32,
}

#[derive(Debug)]
pub struct RegisterRequestPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub enum Msg {
    LoginRequest(LoginRequestPayload),
    LoginResponse(Result<LoginSuccessPayload, ErrorPayload>),
    RegisterRequest(RegisterRequestPayload),
    RegisterResponse(Result<(), ErrorPayload>),
}
