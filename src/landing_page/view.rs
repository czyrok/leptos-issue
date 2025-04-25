use leptos::prelude::*;

use crate::{landing_page::section::Section, shared::{button_action::ButtonAction, my_primary_button::MyPrimaryButton}};

#[component]
pub fn MyView() -> impl IntoView {
    view! {
        <Section />
    }
}
