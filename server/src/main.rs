use api::AppState;
use app::*;
use axum::extract::Path;
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response as AxumResponse};
use axum::routing::{get, post};
use axum::{
    Router,
    extract::{Request, State},
};
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{
    LeptosRoutes, generate_route_list, generate_route_list_with_exclusions_and_ssg_and_context,
    handle_server_fns_with_context,
};
use migrator::migrate;

async fn leptos_routes_handler(state: State<AppState>, req: Request) -> AxumResponse {
    let state1 = state.0.clone();
    let options2 = state.clone().0.options.clone();
    let handler = leptos_axum::render_route_with_context(
        state.routes.clone().unwrap(),
        move || {
            provide_context(state1.clone());
            provide_context(state1.options.clone());
            provide_context(state1.server_signals.clone());
        },
        move || shell(options2.clone()),
    );
    handler(state, req).await.into_response()
}
async fn server_fn_handler(
    State(state): State<AppState>,
    _path: Path<String>,
    _headers: HeaderMap,
    _query: axum::extract::RawQuery,
    request: Request,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context(state.clone());
            provide_context(state.options.clone());
            provide_context(state.server_signals.clone());
        },
        request,
    )
    .await
}
#[tokio::main]
async fn main() {
    // Set up logging
    simple_logger::init_with_level(log::Level::Info).unwrap();

    dotenvy::dotenv().ok();
    let uri = std::env::var("DATABASE_URL").unwrap();
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    let mut state = AppState::new(&uri, conf.leptos_options.clone(), None).await;
    match migrate(&mut state.pool.clone()).await {
        Ok(_) => log::info!("Database migration completed successfully."),
        Err(e) => log::error!("Database migration failed: {:?}", e),
    }

    let leptos_options = conf.leptos_options;
    leptos_captcha::spow::pow::Pow::init_random().unwrap();
    // Generate the list of routes in your Leptos App
    let state2 = state.clone();

    let (routes, _) = generate_route_list_with_exclusions_and_ssg_and_context(
        || view! { <App/> },
        None,
        move || provide_context(state2.server_signals.clone()),
    );
    state.routes = Some(routes.clone());
    let state3 = state.clone();

    let session_store = api::get_session_store(state.pool.clone()).await;
    let pool = state.pool.clone();
    leptos_styling::generate_style_sheets(leptos_options.clone());
    let layer = api::get_auth_session(pool.clone()).await;
    let app = Router::new()
        .route("/api/{*fn_name}", post(server_fn_handler))
        .route("/api/{*fn_name}", get(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(leptos_axum::file_and_error_handler_with_context::<
            AppState,
            _,
        >(
            move || {
                provide_context(state3.server_signals.clone());
                provide_context(state3.clone());
            },
            shell,
        ))
        .with_state(state)
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
