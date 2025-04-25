use leptos::prelude::provide_context;
use leptos_axum::{
    generate_route_list_with_exclusions_and_ssg_and_context, AxumRouteListing,
    StaticRouteGenerator,
};

use super::{super::state::app_state::AppState, shell::shell};

pub fn get_static_route_generator(
    app_state: AppState,
) -> (Vec<AxumRouteListing>, StaticRouteGenerator) {
    generate_route_list_with_exclusions_and_ssg_and_context(
        move || shell(app_state.options.clone()),
        vec![].into(),
        move || {
        },
    )
}
