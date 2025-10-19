use components::header::Header;
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path,
};
use leptos_styling::StyleSheets;

use crate::{
    contexts::{account_context::AccountProvider, theme_context::ThemeContextProvider},
    pages::{login::LoginPage, signup::SignupPage},
};

mod components;
mod contexts;
mod pages;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
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

    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    leptos_ws::provide_websocket();
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
                                <Route path=path!("chat") view=move || view!{
                                    <main class="chat-area">
                                        <pages::chat::ChatPage/>
                                    </main>
                                }/>

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
    view! {}
}
