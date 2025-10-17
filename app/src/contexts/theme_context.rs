use leptos::{context::Provider, prelude::*};
use leptos_use::{
    use_color_mode_with_options, use_cycle_list_with_options, ColorMode, UseColorModeOptions,
    UseColorModeReturn, UseCycleListOptions, UseCycleListReturn,
};
#[derive(Debug, Clone, Copy)]
pub struct ThemeContext {
    next: Callback<()>,
    mode: Signal<ColorMode>,
    set_mode: WriteSignal<ColorMode>,
}

impl ThemeContext {
    pub fn new() -> Self {
        let UseColorModeReturn { mode, set_mode, .. } =
            use_color_mode_with_options(UseColorModeOptions::default().attribute("data-theme"));

        let UseCycleListReturn { next, .. } = use_cycle_list_with_options(
            vec![ColorMode::Light, ColorMode::Dark],
            UseCycleListOptions::default().initial_value(Some((mode, set_mode).into())),
        );
        let next = Callback::from(next);
        Self {
            next,
            mode,
            set_mode,
        }
    }

    pub fn next(&self) {
        self.next.run(());
    }
}

impl AsRef<Signal<ColorMode>> for ThemeContext {
    fn as_ref(&self) -> &Signal<ColorMode> {
        &self.mode
    }
}

impl AsRef<WriteSignal<ColorMode>> for ThemeContext {
    fn as_ref(&self) -> &WriteSignal<ColorMode> {
        &self.set_mode
    }
}

pub fn use_theme_context() -> ThemeContext {
    let context = use_context::<ThemeContext>().expect("ThemeContextProvider not found");
    context
}

#[component]
pub fn ThemeContextProvider(children: Children) -> impl IntoView {
    let theme = ThemeContext::new();
    view! {
        <Provider value={theme}>
            {children()}
        </Provider>
    }
}
