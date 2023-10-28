use std::env;

use anyhow::Context;
use axum::{
    extract::{Query, State},
    http::{header::SET_COOKIE, header::USER_AGENT, HeaderMap},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    error::AppError,
    html::{AppState, COOKIE_NAME},
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
struct User {
    id: i32,
    avatar_url: Option<String>,
    login: String,
    html_url: String,
}

async fn login_authorized(
    Query(query): Query<AuthRequest>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let token = state
        .oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .context("failed in sending request request to authorization server")?;

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/user")
        .bearer_auth(token.access_token().secret())
        .header(USER_AGENT, "oauth_test")
        .send()
        .await
        .context("failed in sending request to target Url")?;

    let response_text = response.text().await.unwrap();

    info!("{}", response_text);
    let user_data: User = serde_json::from_str(&response_text)?;

    // TODO: save user data in db?

    let cookie = format!("{COOKIE_NAME}={}; SameSite=Lax; Path=/", user_data.id);

    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );

    Ok((headers, Redirect::to("/")))
}
