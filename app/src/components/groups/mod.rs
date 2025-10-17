use leptos::prelude::*;

leptos_styling::style_sheet!(
    groups_styles,
    "src/components/groups/groups.module.scss",
    "groups"
);

#[component]
pub fn Groups<T>(children: TypedChildren<T>) -> impl IntoView
where
    T: IntoView + Send + 'static,
{
    view! {
        <div class=groups_styles::GROUPS>
            <h2>"Groups"</h2>
            <div class=groups_styles::GROUP_LIST>
                {(children.into_inner())()}
            </div>
        </div>
    }
}

#[component]
pub fn Group(name: String, last_message: String, picture: String) -> impl IntoView {
    view! {
        <div class=groups_styles::GROUP>
            <img src={picture} alt={name.clone()} />
            <div class=groups_styles::DETAILS>
                <h3>{name}</h3>
                <p>{last_message}</p>
            </div>
        </div>
    }
}
