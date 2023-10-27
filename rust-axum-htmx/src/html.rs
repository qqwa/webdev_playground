use anyhow::Context;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use minijinja::context;
use tracing::info;

use crate::AppState;

mod oauth;

fn app(state: AppState) -> Router {
    Router::new()
        .route("/", get(hello))
        .merge(oauth::router())
        .with_state(state)
}

pub async fn serve(state: AppState) -> anyhow::Result<()> {
    // let app = app(state).into_make_service();
    axum::Server::bind(&"127.0.0.1:4000".parse().unwrap())
        .serve(app(state).into_make_service())
        .await
        .context("failed to start Axum Server")
}

#[derive(Debug)]
struct User2 {
    username: String,
}

async fn hello(State(state): State<AppState>) -> impl IntoResponse {
    let user = sqlx::query_as!(User2, "SELECT username from users")
        .fetch_one(&state.db)
        .await
        .unwrap();
    info!("{:?}", user);

    let env = state.reloader.acquire_env().unwrap();
    let template = env.get_template("index.html").unwrap();
    let username = user.username;
    let render = template.render(context! {username}).unwrap();
    Html(render)
}