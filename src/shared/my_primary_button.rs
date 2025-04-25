use leptos::prelude::*;

use crate::shared::my_button::MyButton;

use super::button_action::ButtonAction;

#[component]
pub fn MyPrimaryButton(
    #[prop(into)] on_click: ButtonAction,
    #[prop(into, default = "".into())] anchor_name: String,
    #[prop(optional, into)] shows_ping: Option<Signal<bool>>,
) -> impl IntoView {
    view! {
        <MyButton on_click anchor_name shows_ping />
    }
}
