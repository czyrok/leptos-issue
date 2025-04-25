use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    path,
    static_routes::StaticRoute,
    SsrMode,
};

use crate::landing_page::view::MyView;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();


    view! {
        <Title text="Welcome to Leptos"/>

        <Router>
            <FlatRoutes fallback=|| {
                view! {
                    "not found"
                }
                .into_view()
            }>
                <Route
                    path=path!("/")
                    view=MyView
                    ssr=SsrMode::Static(StaticRoute::new())
                />
            </FlatRoutes>
        </Router>
    }
}
