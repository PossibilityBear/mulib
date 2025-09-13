use axum::{body::Body, extract::{Request, State}, response::{IntoResponse, Response}};
use leptos::{prelude::provide_context, view};
use leptos_axum::{handle_server_fns_with_context, render_app_to_stream_with_context};

use crate::components::app::App;

pub mod components;
pub mod models;
pub mod database;
pub mod app_state;


#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::routing::get;
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use crate::app_state::AppState;
    use crate::components::app::{shell, App};
    use crate::database::utils::{
        db_connection::*, migrate::migrate
    };
    use crate::database::commands::initialize::initialize_db;
    use tower_http::services::ServeDir;

    println!("STARTING --- database setup ---");
    // set up our database
    let conn = DbConnection::new().await;
    migrate(&conn).await;
    _ = initialize_db(&conn).await.expect("Db to initalize");

    println!("COMPLETE --- database setup--- ");

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Initialize AppState
    let state = AppState { leptos_options, db: conn };


    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/api/{*fn_name}", get(server_fn_handler).post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler) )
        .nest_service("/music", ServeDir::new("music"))
        .nest_service("/public", ServeDir::new("public"))
        .fallback::<_, (_, _, axum::extract::State<AppState>, axum::http::Request<axum::body::Body>)>(leptos_axum::file_and_error_handler(shell))
        .with_state(state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}

#[cfg(feature = "ssr")]
async fn server_fn_handler (
    State(app_state): State<app_state::AppState>,
    req: Request<Body>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context(app_state.clone());
        },
        req
    ).await
}


#[cfg(feature = "ssr")]
async fn leptos_routes_handler(
    State(app_state): State<app_state::AppState>,
    req: Request<Body>,
) -> Response {
    use crate::components::app::shell;
    let opt = app_state.leptos_options.clone();
    let handler = render_app_to_stream_with_context(
        move || {
            provide_context(app_state.clone());
        },
        move || {
            shell(opt.clone())
        }
    );
    handler(req).await.into_response()
}
