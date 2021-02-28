use authentication::RegisterRequestPayload;
use iced::{Button, Column, Element, Row, Text, TextInput};

use crate::{
    messages::{authentication, routing, ui, Msg},
    state::{Model, Route},
};

pub fn view(state: &mut Model) -> Element<Msg> {
    if state.ui.register_screen.loading {
        return Text::new("loading...").into();
    }

    let username_input = TextInput::new(
        &mut state.ui.register_screen.username_input_state,
        "",
        &state.ui.register_screen.username_input_value,
        |val| {
            Msg::Ui(ui::Msg::Register(ui::register::Msg::UsernameInputChanged(
                val,
            )))
        },
    );

    let password_input = TextInput::new(
        &mut state.ui.register_screen.password_input_state,
        "",
        &state.ui.register_screen.password_input_value,
        |val| {
            Msg::Ui(ui::Msg::Register(ui::register::Msg::PasswordInputChanged(
                val,
            )))
        },
    )
    .password();

    let register_button = Button::new(
        &mut state.ui.register_screen.submit_button_state,
        Text::new("Go"),
    )
    .on_press(Msg::Authentication(authentication::Msg::RegisterRequest(
        RegisterRequestPayload {
            username: state.ui.register_screen.username_input_value.to_owned(),
            password: state.ui.register_screen.password_input_value.to_owned(),
        },
    )));

    let login_button = Button::new(
        &mut state.ui.register_screen.login_button_state,
        Text::new("Login"),
    )
    .on_press(Msg::Routing(routing::Msg::Navigate(Route::Login)));

    Column::new()
        .push(Row::new().push(Text::new("Register")))
        .push(Row::new().push(username_input))
        .push(Row::new().push(password_input))
        .push(Row::new().push(register_button))
        .push(
            Row::new()
                .push(Text::new("Already have an account? "))
                .push(login_button),
        )
        .into()
}
