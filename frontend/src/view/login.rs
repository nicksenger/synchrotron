use authentication::LoginRequestPayload;
use iced::{Button, Column, Element, Row, Text, TextInput};

use crate::{
    messages::{authentication, routing, ui, Msg},
    state::{Model, Route},
};

pub fn view(state: &mut Model) -> Element<Msg> {
    if state.ui.login_screen.loading {
        return Text::new("loading...").into();
    }

    let username_input = TextInput::new(
        &mut state.ui.login_screen.username_input_state,
        "",
        &state.ui.login_screen.username_input_value,
        |val| Msg::Ui(ui::Msg::Login(ui::login::Msg::UsernameInputChanged(val))),
    );

    let password_input = TextInput::new(
        &mut state.ui.login_screen.password_input_state,
        "",
        &state.ui.login_screen.password_input_value,
        |val| Msg::Ui(ui::Msg::Login(ui::login::Msg::PasswordInputChanged(val))),
    )
    .password();

    let login_button = Button::new(
        &mut state.ui.login_screen.submit_button_state,
        Text::new("Go"),
    )
    .on_press(Msg::Authentication(authentication::Msg::LoginRequest(
        LoginRequestPayload {
            username: state.ui.login_screen.username_input_value.to_owned(),
            password: state.ui.login_screen.password_input_value.to_owned(),
        },
    )));

    let register_button = Button::new(
        &mut state.ui.login_screen.register_button_state,
        Text::new("Register"),
    )
    .on_press(Msg::Routing(routing::Msg::Navigate(Route::Register)));

    Column::new()
        .push(Row::new().push(Text::new("Login")))
        .push(Row::new().push(username_input))
        .push(Row::new().push(password_input))
        .push(Row::new().push(login_button))
        .push(
            Row::new()
                .push(Text::new("No account? "))
                .push(register_button),
        )
        .into()
}
