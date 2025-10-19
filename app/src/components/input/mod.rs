use leptos::prelude::*;

leptos_styling::style_sheet!(
    input_styles,
    "src/components/input/input.module.scss",
    "input"
);

#[component]
pub fn InputField(
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] maxlength: Option<usize>,
    #[prop(optional)] input_type: Option<&'static str>,
    #[prop(optional)] error: Signal<bool>,
    #[prop(optional)] no_bottom_margin: bool,
    #[prop(optional, into)] value: RwSignal<String>,
) -> impl IntoView {
    let id_value = id.unwrap_or_else(|| name.unwrap_or_default());
    view! {
            <div class=input_styles::INPUT_CONTAINER
                class=(input_styles::NO_BOTTOM_MARGIN, no_bottom_margin)
            >
                {label.map(|text| view! {
                    <label
                        class=input_styles::LABEL
                        for=id_value
                    >
                        {text}
                    </label>
                })}
                <input
                    class=(input_styles::INPUT_FIELD,true)
                    class=(input_styles::ERROR, move || error.get())
                    name=name
                    id=id_value
                    type=input_type
                    maxlength=maxlength
                        .map(|len| len.to_string())
                    placeholder=placeholder
                    prop:value=move || value.get()
                    on:input=move |event| {
                        let input = event_target_value(&event);
                        value.set(input);
                    }
                />
                </div>
    }
}

#[component]
pub fn HiddenField(#[prop(into)] name: String, #[prop(into)] value: String) -> impl IntoView {
    view! {
        <input
            type="hidden"
            name=name
            prop:value=value
        />
    }
}
