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

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubResponseRepository {
    pub id: i32,
    pub node_id: String,
    pub name: Option<String>,
    pub full_name: String,
    pub owner: Option<GithubResponseUser>,
    pub private: bool,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
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
        let response = self.do_request("/user").await?;
        let user: GithubResponseUser = response.json().await?;
        Ok(user)
    }

    pub async fn get_public_repositories(&self) -> Result<Vec<GithubResponseRepository>, AppError> {
        let response = self.do_request("/user/repos").await?;
        let repos: Vec<GithubResponseRepository> = response.json().await?;
        Ok(repos)
    }

    async fn do_request(&self, endpoint: &str) -> Result<Response, AppError> {
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}{}", "https://api.github.com", endpoint))
            .bearer_auth(&self.auth_token)
            .header(USER_AGENT, "oauth_test")
            .send()
            .await
            .context("failed in sending request to target Url")?;

        Ok(response)
    }
}
