use leptos::prelude::*;
leptos_styling::style_sheet!(
    dialog_style,
    "src/components/dialog/dialog.module.scss",
    "dialog"
);
#[component]
pub fn Dialog(#[prop(into)] open: Signal<bool>, children: Children) -> impl IntoView {
    let dialog_node = NodeRef::<leptos::html::Dialog>::new();
    Effect::new(move || {
        if let Some(dialog) = dialog_node.get() {
            if open.get() {
                dialog.show_modal();
            } else {
                dialog.close();
            }
        }
    });
    view! {
        <dialog node_ref=dialog_node class=dialog_style::DIALOG>
            {children()}
        </dialog>
    }
}
