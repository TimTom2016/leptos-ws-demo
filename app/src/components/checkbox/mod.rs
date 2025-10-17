use leptos::prelude::*;

leptos_styling::style_sheet!(
    checkbox_styles,
    "src/components/checkbox/checkbox.module.scss",
    "checkbox"
);

#[component]
pub fn Checkbox(
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] checked: Option<Signal<bool>>,
    #[prop(optional)] on_change: Option<Callback<bool>>,
    #[prop(optional)] error: Option<Signal<bool>>,
    #[prop(optional)] disabled: Option<bool>,
) -> impl IntoView {
    let id_value = id
        .clone()
        .unwrap_or_else(|| name.clone().unwrap_or("checkbox"));

    let is_error = error.unwrap_or_else(|| Signal::derive(|| false));
    let is_disabled = disabled.unwrap_or(false);

    // Create a local signal if none was provided
    let (value, set_value) = signal(checked.map(|c| c.get()).unwrap_or(false));
    let is_checked = move || checked.map_or_else(|| value.get(), |c| c.get());

    let handle_change = move |ev| {
        let checked_value = event_target_checked(&ev);
        set_value.set(checked_value);
        if let Some(cb) = on_change {
            cb.run(checked_value);
        }
    };

    view! {
        <div
            class=checkbox_styles::CHECKBOX_CONTAINER
            class=(checkbox_styles::ERROR, move || is_error.get())
        >
            <label for=id_value.clone() class=checkbox_styles::CHECKBOX_CONTAINER>
                <input
                    id=id_value
                    type="checkbox"
                    name=name
                    prop:checked=is_checked
                    on:change=handle_change
                    disabled=is_disabled
                    class=checkbox_styles::CHECKBOX_INPUT
                />
                <span class=checkbox_styles::CHECKBOX_CUSTOM></span>
                {label.map(|text| view! {
                    <span class=checkbox_styles::CHECKBOX_LABEL>
                        {text}
                    </span>
                })}
            </label>
        </div>
    }
}
