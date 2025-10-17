use api::AppState;
use app::*;
use axum::Router;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use migrator::migrate;
#[tokio::main]
async fn main() {
    // Set up logging
    simple_logger::init_with_level(log::Level::Info).unwrap();

    dotenvy::dotenv().ok();
    let uri = std::env::var("DATABASE_URL").unwrap();
    let state = AppState::new(&uri).await;
    match migrate(&mut state.pool.clone()).await {
        Ok(_) => log::info!("Database migration completed successfully."),
        Err(e) => log::error!("Database migration failed: {:?}", e),
    }
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    leptos_captcha::spow::pow::Pow::init_random().unwrap();
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    let session_store = api::get_session_store(state.pool.clone()).await;
    let pool = state.pool.clone();
    leptos_styling::generate_style_sheets(leptos_options.clone());
    let layer = api::get_auth_session(pool.clone()).await;
    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(state.clone()),
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        .layer(layer)
        .layer(axum_session::SessionLayer::new(session_store));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
