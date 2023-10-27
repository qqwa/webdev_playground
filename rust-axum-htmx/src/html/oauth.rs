use axum::{response::IntoResponse, routing::get, Router};

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/auth/callback/github", get(github_auth))
}

async fn github_auth() -> impl IntoResponse {
    "github auth"
}
