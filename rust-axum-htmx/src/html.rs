use anyhow::Context;
use async_session::log::warn;
use axum::{
    error_handling::HandleErrorLayer,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect},
    routing::get,
    Form, Router,
};
use axum_csrf::{CsrfConfig, CsrfLayer, CsrfToken};
use oauth2::basic::BasicClient;
use serde::Deserialize;
use sqlx::PgPool;
use tower::{BoxError, ServiceBuilder};
use tower_sessions::Session;
use tracing::info;

use crate::{
    error::AppError,
    github::Github,
    html::session::{Counter, SessionUser},
    models::User,
    views::{BoxTemplate, FormTemplate, IndexTemplate, RepoInfo, ReposTemplate, SecretTemplate},
};

pub mod oauth;
mod session;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub oauth_client: BasicClient,
}

fn app(state: AppState, csrf: CsrfConfig) -> Router {
    let db = state.db.clone();
    Router::new()
        .route("/", get(hello))
        .route("/session", get(session))
        .route("/form", get(form_get).post(form_post))
        .route("/repos", get(repos))
        .route("/secret", get(secret))
        .route("/box", get(box_get))
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
        .layer(CsrfLayer::new(csrf))
}

pub async fn serve(state: AppState, csrf: CsrfConfig) -> anyhow::Result<()> {
    // let app = app(state).into_make_service();
    axum::Server::bind(&"127.0.0.1:4000".parse().unwrap())
        .serve(app(state, csrf).into_make_service())
        .await
        .context("failed to start Axum Server")
}

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
            let user: Option<User> = User::get_user_by_github_id(&state.db, github_id).await?;
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

    Ok(IndexTemplate {
        count,
        github_id,
        username: &username,
    }
    .into_response())
}

async fn session(session: Session) -> Result<impl IntoResponse, AppError> {
    Ok(format!("{:#?}", session))
}

async fn form_get(csrf_token: CsrfToken) -> Result<impl IntoResponse, AppError> {
    let template = FormTemplate {
        csrf_token: csrf_token.authenticity_token()?,
        ..Default::default()
    };
    Ok((csrf_token, template))
}

#[derive(Deserialize, Debug)]
struct FormData {
    csrf_token: String,
    data: String,
}

async fn form_post(
    csrf_token: CsrfToken,
    Form(form_data): Form<FormData>,
) -> Result<impl IntoResponse, AppError> {
    match csrf_token.verify(&form_data.csrf_token) {
        Ok(_) => Ok(FormTemplate {
            csrf_token: csrf_token.authenticity_token()?,
            response: Some(form_data.data),
        }),
        Err(_) => Ok(FormTemplate {
            csrf_token: csrf_token.authenticity_token()?,
            response: Some("Wrong csrf token...".into()),
        }),
    }
}

async fn repos(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let session_user: SessionUser = session
        .get(SessionUser::key())
        .expect("could not serialize")
        .unwrap_or_default();

    if let SessionUser::Github(github_id) = session_user {
        let user: User = User::get_user_by_github_id(&state.db, github_id)
            .await?
            .unwrap();

        let repos = Github::new(&user.access_token.unwrap_or_default())
            .get_public_repositories()
            .await?;

        let repos: Vec<RepoInfo> = repos
            .into_iter()
            .map(|repo| RepoInfo {
                full_name: repo.full_name,
                description: repo.description.unwrap_or_default(),
                url: repo.html_url,
            })
            .collect();

        Ok(ReposTemplate { repos }.into_response())
    } else {
        Ok(().into_response())
    }
}

async fn secret(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let session_user: SessionUser = session
        .get(SessionUser::key())
        .expect("could not serialize")
        .unwrap_or_default();

    if let SessionUser::Github(github_id) = session_user {
        let user = User::get_user_by_github_id(&state.db, github_id)
            .await?
            .unwrap();
        return Ok(SecretTemplate {
            username: user.github_login.unwrap(),
            box_template: BoxTemplate {
                color: "bg-yellow-200".to_owned(),
            },
        }
        .into_response());
    } else {
        return Ok(Redirect::to("/").into_response());
    }
}

async fn box_get(headers: HeaderMap) -> Result<impl IntoResponse, AppError> {
    let old_color: Option<&str> = headers
        .get("HX-Trigger-Name")
        .map(|header| header.to_str().unwrap());
    let color = match old_color {
        Some("bg-yellow-200") => "bg-blue-200".to_owned(),
        Some("bg-blue-200") => "bg-green-200".to_owned(),
        Some("bg-green-200") => "bg-gray-200".to_owned(),
        Some("bg-gray-200") => "bg-violet-200".to_owned(),
        Some("bg-violet-200") => "bg-red-200".to_owned(),
        Some("bg-red-200") => "bg-indigo-200".to_owned(),
        _ => "bg-yellow-200".to_owned(),
    };

    Ok(BoxTemplate { color }.into_response())
}
