use anyhow::Context;
use reqwest::{header::USER_AGENT, Response};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubResponseUser {
    pub id: i32,
    pub avatar_url: Option<String>,
    pub login: String,
    pub html_url: String,
}

pub struct Github {
    auth_token: String,
}

impl Github {
    pub fn new(auth_token: &str) -> Self {
        Github {
            auth_token: auth_token.into(),
        }
    }

    pub async fn get_authenticated_user(&self) -> Result<GithubResponseUser, AppError> {
        let response = self.do_request("https://api.github.com/user").await?;
        let user: GithubResponseUser = response.json().await?;
        Ok(user)
    }

    async fn do_request(&self, url: &str) -> Result<Response, AppError> {
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .bearer_auth(&self.auth_token)
            .header(USER_AGENT, "oauth_test")
            .send()
            .await
            .context("failed in sending request to target Url")?;

        Ok(response)
    }
}
