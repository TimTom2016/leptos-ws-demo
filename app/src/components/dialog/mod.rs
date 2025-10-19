use leptos::{attr::custom::custom_attribute, prelude::*};
use leptos_use::on_click_outside;
leptos_styling::style_sheet!(
    dialog_style,
    "src/components/dialog/dialog.module.scss",
    "dialog"
);
#[component]
pub fn Dialog(
    #[prop(into)] open: Signal<bool>,
    children: Children,
    #[prop(into, optional)] on_outside_click: Option<Callback<()>>,
) -> impl IntoView {
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
    Effect::new(move |_| {
        let stop = on_click_outside(dialog_node, move |_| {
            if *open.read()
                && let Some(on_outside_click) = on_outside_click
            {
                on_outside_click.run(());
            }
        });
        on_cleanup(stop);
    });
    view! {
        <dialog node_ref=dialog_node class=dialog_style::DIALOG on:close=move |_| {
            if open.get() && let Some(on_outside_click) = on_outside_click {
                on_outside_click.run(());
            }
        }>
            {children()}
        </dialog>
    }
    .add_any_attr(custom_attribute("closedby", "any"))
}

#[component]
pub fn DialogActions<T>(children: TypedChildren<T>) -> impl IntoView
where
    T: IntoView + Send + 'static,
{
    view! {
        <div class=dialog_style::ACTIONS>
            {(children.into_inner())()}
        </div>
    }
}

#[component]
pub fn DialogHeader<T>(children: TypedChildren<T>) -> impl IntoView
where
    T: IntoView + Send + 'static,
{
    view! {
        <div class=dialog_style::HEADER>
            {(children.into_inner())()}
        </div>
    }
}

#[component]
pub fn DialogBody<T>(children: TypedChildren<T>) -> impl IntoView
where
    T: IntoView + Send + 'static,
{
    view! {
        <div class=dialog_style::BODY>
            {(children.into_inner())()}
        </div>
    }
}

#[component]
pub fn DialogFooter<T>(children: TypedChildren<T>) -> impl IntoView
where
    T: IntoView + Send + 'static,
{
    view! {
        <div class=dialog_style::FOOTER>
            {(children.into_inner())()}
        </div>
    }
}
