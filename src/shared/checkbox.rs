use leptos::prelude::*;

#[component]
pub fn Checkbox(
    text: String,
    #[prop(name = "value")] (is_checked, set_is_checked): (
        ReadSignal<bool>,
        WriteSignal<bool>,
    ),
    #[prop(default = true)] can_user_unchecked: bool,
) -> impl IntoView {

    view! {
        <label
            class="tw-secondary-checkbox"
        >
            <input
                class="tw-checkbox-input"
                type="checkbox"

                // FIXME:
                // on:input:target=move |event| {
                //     let is_checked = event.target().checked();

                //     if is_checked {
                //         set_is_checked.set(is_checked);

                //         return;
                //     }

                //     if can_user_unchecked {
                //         set_is_checked.set(is_checked);
                //     } else {
                //         //// We need to restore the checked view
                //         set_is_checked.set(true);
                //     }
                // }
                // prop:checked=is_checked
            />

            <div class="tw-checkbox-box">
                <span class="tw-box-icon">
                    "icon"
                </span>
            </div>

            <span class="tw-checkbox-text">{ text }</span>
        </label>
    }
}
