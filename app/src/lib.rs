use components::header::Header;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path, StaticSegment,
};
use leptos_styling::StyleSheets;
use thaw::ssr::SSRMountStyleProvider;

use crate::{
    contexts::{account_context::AccountProvider, theme_context::ThemeContextProvider},
    pages::{login::LoginPage, signup::SignupPage},
};

mod components;
mod contexts;
mod pages;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <SSRMountStyleProvider>
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" href="/512x512-Maskable.svg" sizes="any" type="image/svg+xml"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
        </SSRMountStyleProvider>

    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-ws-demo.css"/>

        <Title text="Leptos Chat"/>
        <StyleSheets base_url="/pkg"/>
        <Router>
            <ThemeContextProvider>
                <AccountProvider>
                    <Header>
                            <Routes fallback=|| "Page not found.".into_view()>
                                <Route path=path!("") view=HomePage/>
                                <Route path=path!("login") view=LoginPage />
                                <Route path=path!("signup") view=SignupPage />
                            </Routes>
                    </Header>
                </AccountProvider>
            </ThemeContextProvider>

        </Router>

    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <main class="chat-area">
            <pages::home::HomePage/>

        </main>

    }
}
