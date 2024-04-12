/*
 * Copyright (C) 2024 Kirill Lukashev <kirill.lukashev.sic@gmail.com>,
 * Gleb Krylov <gleb_cry@mail.ru>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use std::collections::HashMap;

use axum::body::Body as AxumBody;
use axum::extract::{FromRef, State};
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use cicero_app::server::Env;
use cicero_app::App;
use cicero_dsl::compiler::compile_scenario;
use cicero_dsl::scenario::Scenario;
use leptos::*;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};

use self::fileserv::file_and_error_handler;

mod fileserv;

#[derive(FromRef, Debug, Clone)]
struct AppState {
    env: Env,
    leptos_options: LeptosOptions,
}

async fn compile_scenarios(path: impl AsRef<std::path::Path>) -> HashMap<u64, Scenario> {
    let mut dirs = tokio::fs::read_dir(path).await.unwrap();
    let mut scenarios = HashMap::new();

    while let Some(entry) = dirs.next_entry().await.unwrap() {
        let path = entry.path();
        if path.is_dir() {
            let scenario = tokio::task::spawn_blocking(move || compile_scenario(path))
                .await
                .unwrap()
                .unwrap();
            let id = scenario.meta().id;
            scenarios.insert(id, scenario);
        }
    }

    scenarios
}

async fn server_fn_handler(
    State(app_state): State<AppState>,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context(app_state.env.clone());
        },
        request,
    )
    .await
}

async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    req: Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_app_to_stream_with_context(
        app_state.leptos_options.clone(),
        move || {
            provide_context(app_state.env.clone());
        },
        || view! { <App/> },
    );
    handler(req).await.into_response()
}

#[tokio::main]
async fn main() {
    // NB:
    // Setting get_configuration(None) means we'll be using cargo-leptos's env
    // values. For deployment these variables see:
    //
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    //
    // Alternately a file can be specified such as Some("Cargo.toml").
    // The file would need to be included with the executable when moved to
    // deployment.
    let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let scenarios = compile_scenarios("scenarios").await;

    let app_state = AppState {
        env: Env::new(scenarios),
        leptos_options: leptos_options.clone(),
    };

    // build our application with a route
    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
