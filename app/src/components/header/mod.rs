use api::server_fn::logout::Logout;
use leptos::prelude::*;
use leptos_router::components::A;

use crate::{
    components::{
        dropdown::{Dropdown, dropdown_styles},
        theme_switcher::ThemeSwitcher,
    },
    contexts::account_context::AccountContext,
};

leptos_styling::style_sheet!(
    header_styles,
    "src/components/header/header.module.scss",
    "header"
);
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Page {
    Home,
    Explore,
    Notifications,
    Messages,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct HeaderContext {
    current_page: RwSignal<Page>,
}

impl HeaderContext {
    pub fn new(current_page: Page) -> Self {
        Self {
            current_page: RwSignal::new(current_page),
        }
    }

    pub fn switch_page(&self, new_page: Page) {
        self.current_page.set(new_page);
    }

    pub fn page_read(&self) -> ReadSignal<Page> {
        self.current_page.read_only()
    }
}

#[component]
pub fn Header<T: IntoView>(children: TypedChildren<T>) -> impl IntoView {
    let context = HeaderContext::new(Page::Home);
    let current_page = context.page_read();
    let account = use_context::<AccountContext>().expect("AccountContext not found");
    let logout = ServerAction::<Logout>::new();
    provide_context(context);
    Effect::new(move |_| {
        if *logout.version().read() > 0 {
            account.refresh();
        }
    });
    view! {
        <header class=header_styles::HEADER>
          <div class=header_styles::LOGO>
            <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M24 45.8096C19.6865 45.8096 15.4698 44.5305 11.8832 42.134C8.29667 39.7376 5.50128 36.3314 3.85056 32.3462C2.19985 28.361 1.76794 23.9758 2.60947 19.7452C3.451 15.5145 5.52816 11.6284 8.57829 8.5783C11.6284 5.52817 15.5145 3.45101 19.7452 2.60948C23.9758 1.76795 28.361 2.19986 32.3462 3.85057C36.3314 5.50129 39.7376 8.29668 42.134 11.8833C44.5305 15.4698 45.8096 19.6865 45.8096 24L24 24L24 45.8096Z" fill="currentColor"/>
            </svg>
            <h1>Leptos Chat</h1>
          </div>

          <nav>
            <A href="#" {..} class=(header_styles::ACTIVE,move || current_page.get() == Page::Home)>Home</A>
            <A href="#" {..} class=(header_styles::ACTIVE,move || current_page.get() == Page::Explore)>Explore</A>
            <A href="#" {..} class=(header_styles::ACTIVE,move || current_page.get() == Page::Notifications)>Notifications</A>
            <A href="#" {..} class=(header_styles::ACTIVE,move || current_page.get() == Page::Messages)>Messages</A>
          </nav>

          <div class=header_styles::ACTIONS>
              <ThemeSwitcher/>
            <Dropdown label=move || view!{
                <Suspense>
                    <Show when=move || account.logged_in() fallback=move || view!{
                        <div class=header_styles::AVATAR style=move || format!("background-image: url('https://robohash.org/anonymous');")></div>
                    }>
                        {move || account
                            .user()
                            .and_then(|v| v.username().map(|v| v.to_string())).map(|username| view!{
                            <div class=header_styles::AVATAR style=move || format!("background-image: url('https://robohash.org/{username}');")></div>
                        })}
                    </Show>
                </Suspense>
            }>
                <Suspense>
                    <Show when=move || !account.logged_in()>
                        <A href="login" {..} class=dropdown_styles::DROPDOWN_ITEM>
                            "Login"
                        </A>
                        <A href="signup" {..} class=dropdown_styles::DROPDOWN_ITEM>
                            "Register"
                        </A>
                    </Show>

                    <Show when=move || account.logged_in()>
                        <button on:click=move |_| {logout.dispatch(Logout {});} class=dropdown_styles::DROPDOWN_ITEM>
                            "Logout"
                        </button>
                    </Show>
                </Suspense>
            </Dropdown>
          </div>
        </header>
        {(children.into_inner())()}
    }
}
