use std::env;

use anyhow::Context;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use tower_sessions::Session;

use crate::{
    error::AppError,
    github::Github,
    html::{session::SessionUser, AppState},
    models::User,
};

pub fn oauth_client() -> Result<BasicClient, AppError> {
    let client_id = env::var("CLIENT_ID").context("Missing CLIENT_ID!")?;
    let client_secret = env::var("CLIENT_SECRET").context("Missing CLIENT_SECRET!")?;
    let redirect_url = env::var("REDIRECT_URL").context("Missing REDIRECT_URL!")?;
    let auth_url = env::var("AUTH_URL").context("Missing AUTH_URL!")?;
    let token_url = env::var("TOKEN_URL").context("Missing TOKEN_URL!")?;

    Ok(BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).context("failed to create new authorization server URL")?,
        Some(TokenUrl::new(token_url).context("failed to create new token endpoint URL")?),
    )
    .set_redirect_uri(
        RedirectUrl::new(redirect_url).context("failed to create new redirection URL")?,
    ))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/github", get(github_auth))
        .route("/auth/callback/github", get(login_authorized))
        .route("/auth/logout", get(logout))
}

async fn github_auth(State(state): State<AppState>) -> impl IntoResponse {
    let (auth_url, _csrf_token) = state
        .oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();

    Redirect::to(auth_url.as_ref())
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AuthRequest {
    code: String,
    state: String,
}

#[axum_macros::debug_handler]
async fn login_authorized(
    Query(query): Query<AuthRequest>,
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let token = state
        .oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .context("failed in sending request request to authorization server")?;

    let user = Github::new(token.access_token().secret())
        .get_authenticated_user()
        .await?;

    let db_user: Option<User> = sqlx::query_as!(
        crate::models::User,
        "select * from users where github_id = $1",
        user.id
    )
    .fetch_optional(&state.db)
    .await?;
    let db_user = if db_user.is_none() {
        let new_user: User = sqlx::query_as("insert into users (github_id, github_login, access_token) values($1, $2, $3) returning *")
            .bind(user.id)
            .bind(user.login)
            .bind(token.access_token().secret())
            .fetch_one(&state.db).await?;
        new_user
    } else {
        db_user.unwrap()
    };

    session.insert(SessionUser::key(), SessionUser::Github(db_user.github_id))?;

    Ok(Redirect::to("/"))
}

async fn logout(session: Session) -> Result<impl IntoResponse, AppError> {
    let session_user: SessionUser = session
        .get(SessionUser::key())
        .expect("Could not serialze.")
        .unwrap_or_default();

    match session_user {
        SessionUser::Guest => Ok(Redirect::to("/")),
        SessionUser::Github(_) => {
            session.insert(SessionUser::key(), SessionUser::Guest)?;
            Ok(Redirect::to("/"))
        }
    }
}
