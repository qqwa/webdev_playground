#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub github_id: i32,
    pub github_login: Option<String>,
    pub access_token: Option<String>,
    pub created_at: time::OffsetDateTime,
    pub updated_at: Option<time::OffsetDateTime>,
}
