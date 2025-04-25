use leptos::prelude::*;

use crate::{landing_page::theme_selector::ThemeSelector, shared::{
    button_action::ButtonAction,  my_primary_button::MyPrimaryButton
}};

#[component]
pub fn FooterMorePart(
    middle_action_renderer: Option<Box<dyn Fn() -> AnyView>>,
) -> impl IntoView {
    let middle_action_renderer =
        middle_action_renderer.unwrap_or(Box::new(|| {
            view! {
                <MyPrimaryButton on_click=ButtonAction::None />
                // <SecondaryButtonAsLink size=ComponentSize::MD text="Accueil" href="/" />
            }
            .into_any()
        }));

    view! {
        <div class="tw-footer-more-part">
            {middle_action_renderer()}
        </div>
    }
}

#[component]
pub fn Footer(
    #[prop(default = true)] displays_actions: bool,
    #[prop(optional)] middle_action_renderer: Option<Box<dyn Fn() -> AnyView>>,
) -> impl IntoView {
    let mut more_part_view = view! { "" }.into_any();

    if displays_actions {
        more_part_view = view! {
            <FooterMorePart middle_action_renderer />
        }
        .into_any();
    }

    view! {
        <div class="tw-footer">
            {more_part_view}

            <MyPrimaryButton on_click=|_| {
                leptos::logging::log!("has callback");
            } />

            <ThemeSelector />
        </div>
    }
}
