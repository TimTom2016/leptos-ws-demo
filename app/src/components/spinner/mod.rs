use leptos::prelude::*;

leptos_styling::style_sheet!(
    spinner,
    "src/components/spinner/spinner.module.scss",
    "spinner"
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpinnerSize {
    #[default]
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpinnerColor {
    #[default]
    Primary,
    Secondary,
}

/// A spinner component for loading states
///
/// # Example
/// ```
/// use app::components::spinner::{Spinner, SpinnerSize, SpinnerColor};
///
/// view! {
///     <Spinner size=SpinnerSize::Medium color=SpinnerColor::Primary />
/// }
/// ```
#[component]
pub fn Spinner(
    #[prop(optional)] size: SpinnerSize,
    #[prop(optional)] color: SpinnerColor,
) -> impl IntoView {
    view! {
        <div
            class=spinner::SPINNER
            class=(spinner::SMALL, move || size == SpinnerSize::Small)
            class=(spinner::MEDIUM, move || size == SpinnerSize::Medium)
            class=(spinner::LARGE, move || size == SpinnerSize::Large)
            class=(spinner::PRIMARY, move || color == SpinnerColor::Primary)
            class=(spinner::SECONDARY, move || color == SpinnerColor::Secondary)
        >
            <div class=spinner::DOT></div>
            <div class=spinner::DOT></div>
            <div class=spinner::DOT></div>
        </div>
    }
}
