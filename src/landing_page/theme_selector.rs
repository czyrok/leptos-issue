use leptos::prelude::*;

use crate::{
    shared::{
        dropdown_menu::Position, select::{
            components::select::Select,
            types::{
                select_choice::SelectChoice, single_select::SingleSelectChoices,
            },
        }
    },
    system::state::frontend_contexts::use_app_settings,
};

#[derive(PartialEq, Eq, Clone, Debug)]
enum Theme {
    Light,
    Dark,
}

#[component]
pub fn ThemeSelector() -> impl IntoView {
    let (app_settings, set_app_settings) = use_app_settings();

    let dark_choice = SelectChoice::new(
        Theme::Dark,
        "Sombre".into(),
        Some(app_settings.get_untracked().uses_dark_theme.0),
    );

    dark_choice.attach(move |choice| {
        set_app_settings.update(|app_settings| {
            app_settings.uses_dark_theme.0 = choice.is_checked.get_untracked();
        });
    });

    let light_choice = SelectChoice::new(
        Theme::Light,
        "Clair".into(),
        Some(!app_settings.get_untracked().uses_dark_theme.0),
    );

    let choices = SingleSelectChoices::new([dark_choice, light_choice].into());

    view! {
        <Select dropdown_menu_position=Position::Right text="Thème" select_choices=choices />
    }
}
