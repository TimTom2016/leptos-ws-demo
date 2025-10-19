use components::header::Header;
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{
        Outlet, ParentRoute, ProtectedParentRoute, ProtectedRoute, Route, Router, Routes,
    },
    path,
};
use leptos_styling::StyleSheets;

use crate::{
    components::header::HeaderContext,
    contexts::{
        account_context::{AccountContext, AccountProvider},
        theme_context::ThemeContextProvider,
    },
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
                                <ProtectedRoute condition=move || {
                                    let account_context = expect_context::<AccountContext>();
                                    account_context.user().map(|v| v.is_logged_in())
                                } path=path!("chat") redirect_path=move || "/login?next=/chat" view=move || view!{
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
    let header = expect_context::<HeaderContext>();
    Effect::new(move |_| {
        header.switch_page(components::header::Page::Home);
    });
    view! {
        <main class="home-page">
            <div class="hero-section">
                <h1 class="hero-title">"Welcome to Leptos Chat"</h1>
                <p class="hero-subtitle">"Real-time messaging powered by WebSockets"</p>
                <div class="cta-buttons">
                    <a href="/chat" class="btn btn-primary">"Start Chatting"</a>
                    <a href="/signup" class="btn btn-secondary">"Sign Up"</a>
                </div>
            </div>

            <div class="features-section">
                <h2 class="section-title">"Features"</h2>
                <div class="features-grid">
                    <div class="feature-card">
                        <div class="feature-icon">"⚡"</div>
                        <h3 class="feature-title">"Real-Time"</h3>
                        <p class="feature-description">"Instant message delivery with WebSocket technology"</p>
                    </div>
                    <div class="feature-card">
                        <div class="feature-icon">"🔒"</div>
                        <h3 class="feature-title">"Secure"</h3>
                        <p class="feature-description">"Your conversations are protected and private"</p>
                    </div>
                    <div class="feature-card">
                        <div class="feature-icon">"🚀"</div>
                        <h3 class="feature-title">"Fast"</h3>
                        <p class="feature-description">"Built with Rust and Leptos for blazing speed"</p>
                    </div>
                </div>
            </div>
        </main>

        <style>
            {r#"
                .home-page {
                    flex-grow: 1;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    padding: 2rem;
                    max-width: 1200px;
                    margin: 0 auto;
                    width: 100%;
                }

                .hero-section {
                    text-align: center;
                    padding: 4rem 1rem;
                    max-width: 800px;
                }

                .hero-title {
                    font-size: 3rem;
                    font-weight: 700;
                    margin-bottom: 1rem;
                    background: linear-gradient(135deg, var(--primary), var(--accent));
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    background-clip: text;
                }

                .hero-subtitle {
                    font-size: 1.5rem;
                    color: var(--text-muted);
                    margin-bottom: 2rem;
                }

                .cta-buttons {
                    display: flex;
                    gap: 1rem;
                    justify-content: center;
                    flex-wrap: wrap;
                }

                .btn {
                    padding: 1rem 2rem;
                    border-radius: var(--radius);
                    text-decoration: none;
                    font-weight: 600;
                    font-size: 1.1rem;
                    transition: all 0.3s ease;
                    display: inline-block;
                }

                .btn-primary {
                    background: var(--primary);
                    color: var(--text-on-primary);
                }

                .btn-primary:hover {
                    background: var(--primary-hover);
                    transform: translateY(-2px);
                    box-shadow: 0 4px 12px rgba(13, 127, 242, 0.3);
                }

                .btn-secondary {
                    background: transparent;
                    color: var(--text-color);
                    border: 2px solid var(--border-color);
                }

                .btn-secondary:hover {
                    border-color: var(--primary);
                    color: var(--primary);
                    transform: translateY(-2px);
                }

                .features-section {
                    width: 100%;
                    padding: 3rem 1rem;
                }

                .section-title {
                    text-align: center;
                    font-size: 2.5rem;
                    font-weight: 700;
                    margin-bottom: 3rem;
                }

                .features-grid {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
                    gap: 2rem;
                    max-width: 1000px;
                    margin: 0 auto;
                }

                .feature-card {
                    background: var(--background);
                    border: 1px solid var(--border-color);
                    border-radius: var(--radius-lg);
                    padding: 2rem;
                    text-align: center;
                    transition: all 0.3s ease;
                }

                .feature-card:hover {
                    transform: translateY(-5px);
                    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
                    border-color: var(--primary);
                }

                .feature-icon {
                    font-size: 3rem;
                    margin-bottom: 1rem;
                }

                .feature-title {
                    font-size: 1.5rem;
                    font-weight: 600;
                    margin-bottom: 0.5rem;
                    color: var(--text-color);
                }

                .feature-description {
                    color: var(--text-muted);
                    line-height: 1.6;
                }

                @media (max-width: 768px) {
                    .hero-title {
                        font-size: 2rem;
                    }

                    .hero-subtitle {
                        font-size: 1.2rem;
                    }

                    .section-title {
                        font-size: 2rem;
                    }

                    .features-grid {
                        grid-template-columns: 1fr;
                    }
                }
            "#}
        </style>
    }
}
