use leptos::{
    html::{Button, Div},
    prelude::*,
};
use leptos_styling::style_sheet;
use leptos_use::{on_click_outside, on_click_outside_with_options, OnClickOutsideOptions};

style_sheet!(
    dropdown_styles,
    "src/components/dropdown/dropdown.module.scss",
    "dropdown"
);

#[component]
pub fn Dropdown<F, IV>(
    label: F,
    children: ChildrenFn,
    #[prop(into, optional)] button_classes: MaybeProp<String>,
    #[prop(into, optional)] content_classes: MaybeProp<String>,
) -> impl IntoView
where
    F: Fn() -> IV + Send + 'static,
    IV: IntoView + Send + 'static,
{
    let target = NodeRef::<Div>::new();
    let open = RwSignal::new(false);
    let button = NodeRef::<Button>::new();
    Effect::new(move |_| {
        let stop = on_click_outside_with_options(
            target,
            move |_| {
                if *open.read() {
                    open.set(false);
                }
            },
            OnClickOutsideOptions::default().ignore(button),
        );
        on_cleanup(stop);
    });

    view! {
        <div class=dropdown_styles::DROPDOWN>
            <button
                node_ref=button
                class=move || if let Some(classes) = button_classes.get() {
                    classes
                } else {
                    dropdown_styles::DROPBTN.to_string()
                }
                on:click=move |_| open.update(|v| *v = !*v)
            >
                {label()}
            </button>

            <Show when=move || open.get()>
                <div class=move || if let Some(classes) = content_classes.get() {
                    classes
                } else {
                    dropdown_styles::DROPDOWN_CONTENT.to_string()
                } node_ref=target on:click=move |_| if *open.read() {
                    open.set(false);
                }>
                    {(children)()}

                </div>
            </Show>
        </div>
    }
}
