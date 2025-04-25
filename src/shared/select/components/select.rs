use leptos::prelude::*;

use crate::{
    shared::{
        checkbox::Checkbox, dropdown_menu::DropdownMenu, my_primary_button::MyPrimaryButton, select::types::select_choices_behavior::SelectChoicesBehavior
    },
    shared::dropdown_menu::Position
};

#[component]
pub fn Select(
    dropdown_menu_position: Position,
    #[prop(into)] text: String,
    select_choices: impl SelectChoicesBehavior,
    #[prop(default = false)] shows_ping_when_least_one_selected: bool,
) -> impl IntoView {
    let converted_choices = select_choices.list();

    select_choices.attach_consistency_behavior();

    let drop_menu_anchor_name = format!("drop-menu-{}", "identifier");
    let button_anchor_name = format!("button-{}", "identifier");

    let shows_ping = select_choices
        .get_selected_choice_keys()
        .map(|selected_choice_keys| {
            if !shows_ping_when_least_one_selected {
                return signal(false).0.into();
            }

            Signal::derive(move || {
                let selected_choice_keys = selected_choice_keys.get();

                !selected_choice_keys.is_empty()
            })
        })
        .unwrap_or(signal(false).0.into());

    view! {
        <MyPrimaryButton on_click=drop_menu_anchor_name.clone() anchor_name=button_anchor_name.clone() shows_ping />

        <DropdownMenu position=dropdown_menu_position id=drop_menu_anchor_name position_anchor_name=button_anchor_name>
            {converted_choices.into_iter()
                .map(|choice| view! {
                    <Checkbox text=choice.text value=(choice.is_checked, choice.set_is_checked) can_user_unchecked=choice.can_user_unchecked />
                })
                .collect::<Vec<_>>()}
        </DropdownMenu>
    }
}
