use leptos::prelude::*;

leptos_styling::style_sheet!(card_styles, "src/components/card/card.module.scss", "card");

#[component]
pub fn Card<T>(
    children: TypedChildren<T>,
    #[prop(optional)] highlight: bool,
    #[prop(optional)] compact: bool,
) -> impl IntoView
where
    T: IntoView + Send + 'static,
{
    view! {
        <div class=(card_styles::CARD,true)
            class=(card_styles::HIGHLIGHT, highlight)
            class=(card_styles::COMPACT, compact)
        >
            {(children.into_inner())()}
        </div>
    }
}

#[component]
pub fn CardActions<T>(children: TypedChildren<T>) -> impl IntoView
where
    T: IntoView + Send + 'static,
{
    view! {
        <div class=card_styles::ACTIONS>
            {(children.into_inner())()}
        </div>
    }
}

#[component]
pub fn CardHeader<T>(children: TypedChildren<T>) -> impl IntoView
where
    T: IntoView + Send + 'static,
{
    view! {
        <div class=card_styles::HEADER>
            {(children.into_inner())()}
        </div>
    }
}

#[component]
pub fn CardBody<T>(children: TypedChildren<T>) -> impl IntoView
where
    T: IntoView + Send + 'static,
{
    view! {
        <div class=card_styles::BODY>
            {(children.into_inner())()}
        </div>
    }
}

#[component]
pub fn CardFooter<T>(children: TypedChildren<T>) -> impl IntoView
where
    T: IntoView + Send + 'static,
{
    view! {
        <div class=card_styles::FOOTER>
            {(children.into_inner())()}
        </div>
    }
}
