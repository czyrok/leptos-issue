use leptos::{ev::MouseEvent, prelude::*};

use super::button_action::ButtonAction;

#[component]
pub fn MyButton(
    #[prop(into)] on_click: ButtonAction,
    #[prop(into, default = "".into())] anchor_name: String,
    #[prop(into)] shows_ping: Option<Signal<bool>>,
) -> impl IntoView {
    let mut on_click_callback: Box<dyn FnMut(MouseEvent)> = Box::new(|_| {
        leptos::logging::log!("no callback");
    });
    let mut popover_target_id: String = "".into();

    match on_click {
        ButtonAction::Callback(callback) => {
            on_click_callback = callback;
        }
        ButtonAction::Popover(target_id) => {
            popover_target_id = target_id;
        }
        _ => (),
    };

    let shows_ping = shows_ping.unwrap_or(signal(false).0.into());

    view! {
        <button
            class=(["tw-button-ping"], move || shows_ping.get())

            on:click=on_click_callback
            popovertarget=popover_target_id
            style=format!("anchor-name: --{}", anchor_name)
        >
            "click"
        </button>
    }
}