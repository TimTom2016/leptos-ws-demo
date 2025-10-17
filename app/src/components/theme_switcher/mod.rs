use std::ops::Deref;

use crate::contexts::theme_context::{use_theme_context, ThemeContext};
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_use::ColorMode;

leptos_styling::style_sheet!(
    theme_switcher_styles,
    "src/components/theme_switcher/theme_switcher.module.scss",
    "theme_switcher"
);

#[component]
pub fn ThemeSwitcher() -> impl IntoView {
    let theme = use_theme_context();
    let toggle_theme = move |_| {
        theme.next();
    };
    let icon = Signal::derive(move || {
        match <ThemeContext as AsRef<Signal<ColorMode>>>::as_ref(&theme).get() {
            ColorMode::Light => icondata::LuSun,
            ColorMode::Dark => icondata::LuMoon,
            _ => icondata::LuSun,
        }
    });
    view! {
        <button class=theme_switcher_styles::THEME_SWITCHER on:click=toggle_theme>
            <Icon icon width="1rem" height="1rem"/>
        </button>
    }
}
