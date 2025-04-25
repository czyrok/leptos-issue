use axum::extract::FromRef;
use leptos::config::LeptosOptions;

// Derive FromRef to allow multiple items in state, using Axum’s SubStates pattern.
#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub options: LeptosOptions,
}

impl AppState {
    pub fn new(
        options: LeptosOptions,
    ) -> Self {
        Self {
            options,
        }
    }
}
