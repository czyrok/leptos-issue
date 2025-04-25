use leptos::prelude::*;

use crate::shared::footer::Footer;

#[component]
pub fn Layout(
    #[prop(optional)] footer_renderer: Option<Box<dyn Fn() -> AnyView>>,
) -> impl IntoView {
    let footer_renderer = footer_renderer.unwrap_or(Box::new(move || {
        view! {
            <Footer />
        }
        .into_any()
    }));

    view! {
        {footer_renderer()}
    }
}
