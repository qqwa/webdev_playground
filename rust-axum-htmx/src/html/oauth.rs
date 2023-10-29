use std::env;

use anyhow::Context;
use axum::{
    extract::{Query, State},
    http::header::USER_AGENT,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::{
    error::AppError,
    html::{session::GithubAccessToken, AppState},
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

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub avatar_url: Option<String>,
    pub login: String,
    pub html_url: String,
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

    session
        .insert(GithubAccessToken::key(), token.access_token().secret())
        .expect("could not store github access token");

    Ok(Redirect::to("/"))
}

pub async fn request_github(url: &str, access_token: &str) -> Result<String, AppError> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .bearer_auth(access_token)
        .header(USER_AGENT, "oauth_test")
        .send()
        .await
        .context("failed in sending request to target Url")?;

    let response_text = response.text().await?;

    Ok(response_text)
}
