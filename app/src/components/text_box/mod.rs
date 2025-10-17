use leptos::prelude::*;
leptos_styling::style_sheet!(
    text_box,
    "src/components/text_box/text_box.module.scss",
    "text-box"
);
#[component]
pub fn TextBox(#[prop(optional)] class: Option<String>, children: Children) -> impl IntoView {
    view! {
        <div class=text_box::TEXT_BOX>
            {children()}
        </div>
    }
}
