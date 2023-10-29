use std::sync::Arc;

use anyhow::Context;
use async_session::log::warn;
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

use crate::{
    error::AppError,
    html::session::{Counter, SessionUser},
    models::User,
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
        .route("/session", get(session))
        .merge(oauth::router())
        .layer(
            //TODO: put whole ServiceBuilder into session mod
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    StatusCode::BAD_REQUEST
                }))
                .layer(session::service(db)),
        )
        .with_state(state)
}

pub async fn serve(state: AppState) -> anyhow::Result<()> {
    // let app = app(state).into_make_service();
    axum::Server::bind(&"127.0.0.1:4000".parse().unwrap())
        .serve(app(state).into_make_service())
        .await
        .context("failed to start Axum Server")
}

#[axum_macros::debug_handler]
async fn hello(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let session_user: SessionUser = session
        .get(SessionUser::key())
        .expect("could not serialize")
        .unwrap_or_default();
    let (github_id, username) = match session_user {
        SessionUser::Guest => (0i32, "".to_owned()),
        SessionUser::Github(github_id) => {
            let user: Option<User> = sqlx::query_as("select * from users where github_id = $1")
                .bind(github_id)
                .fetch_optional(&state.db)
                .await?;
            if let Some(user) = user {
                (user.github_id, user.github_login.unwrap())
            } else {
                warn!(
                    "Session {} has github_id {}, which couldn't be found in database",
                    session.id(),
                    github_id
                );
                (0i32, "error".to_owned())
            }
        }
    };

    let counter: Counter = session
        .get(Counter::key())
        .expect("could not deserizale.")
        .unwrap_or_default();
    let count = counter.0;
    session
        .insert(Counter::key(), count + 1)
        .expect("could not serizale.");

    let env = state.reloader.acquire_env().unwrap();
    let template = env.get_template("index.html").unwrap();
    let render = template
        .render(context! {username, github_id, count})
        .unwrap();
    Ok(Html(render))
}

async fn session(session: Session) -> Result<impl IntoResponse, AppError> {
    Ok(format!("{:#?}", session))
}
