use std::sync::Arc;

use anyhow::Context;
use axum::{
    error_handling::HandleErrorLayer,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use minijinja::context;
use minijinja_autoreload::AutoReloader;
use oauth2::basic::BasicClient;
use sqlx::PgPool;
use tower::{BoxError, ServiceBuilder};
use tower_sessions::Session;
use tracing::info;

use crate::{
    error::AppError,
    html::{oauth::User, session::GithubAccessToken},
};

pub mod oauth;
mod session;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub reloader: Arc<AutoReloader>,
    pub oauth_client: BasicClient,
}

fn app(state: AppState) -> Router {
    let db = state.db.clone();
    Router::new()
        .route("/", get(hello))
        .merge(oauth::router())
        .with_state(state)
        .layer(
            //TODO: put whole ServiceBuilder into session mod
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    StatusCode::BAD_REQUEST
                }))
                .layer(session::service(db)),
        )
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

async fn hello(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let user = sqlx::query_as!(User2, "SELECT username from users")
        .fetch_one(&state.db)
        .await
        .unwrap();
    info!("{:?}", user.username);

    let github_access_token: GithubAccessToken = session
        .get(GithubAccessToken::key())
        .expect("could not deserizale.")
        .unwrap_or_default();
    let (github_id, username) = if github_access_token.0 != "" {
        let response_text =
            oauth::request_github("https://api.github.com/user", &github_access_token.0).await?;
        let user: User = serde_json::from_str(&response_text)?;
        (user.id, user.login)
    } else {
        (0, "".into())
    };

    let env = state.reloader.acquire_env().unwrap();
    let template = env.get_template("index.html").unwrap();
    let render = template.render(context! {username, github_id}).unwrap();
    Ok(Html(render))
}
