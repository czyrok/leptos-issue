use leptos::prelude::*;

use crate::shared::{button_action::ButtonAction, footer::Footer, layout::Layout, my_primary_button::MyPrimaryButton};

#[component]
pub fn Section() -> impl IntoView {
    view! {
        <Suspense fallback=move || {
            view! {
                // <MyPrimaryButton on_click=ButtonAction::None />
                <Layout
                    footer_renderer=Box::new(move || view! {
                        <Footer displays_actions=false />
                    }.into_any())
                />
            }
        }>
            "salut"
        </Suspense>
    }
}