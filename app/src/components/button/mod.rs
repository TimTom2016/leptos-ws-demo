use leptos::prelude::*;
leptos_styling::style_sheet!(
    buttons,
    "src/components/button/button.module.scss",
    "buttons"
);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Tertiary,
    Danger,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Sizing {
    Small,
    #[default]
    Normal,
    Big,
}

#[component]
pub fn Button(
    variant: ButtonVariant,
    #[prop(optional)] sizing: Sizing,
    #[prop(optional)] center: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <button
        class=buttons::BUTTON
        class=(buttons::PRIMARY, move || variant == ButtonVariant::Primary)
        class=(buttons::SECONDARY, move || variant == ButtonVariant::Secondary)
        class=(buttons::DANGER, move || variant == ButtonVariant::Danger)
        class=(buttons::SMALL, move || sizing == Sizing::Small)
        class=(buttons::NORMAL, move || sizing == Sizing::Normal)
        class=(buttons::BIG, move || sizing == Sizing::Big)
        class=(buttons::CENTER, move || center)
        >
            {children()}
        </button>
    }
}
