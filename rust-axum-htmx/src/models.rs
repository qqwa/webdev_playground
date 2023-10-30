use sqlx::PgPool;

use crate::error::AppError;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub github_id: i32,
    pub github_login: Option<String>,
    pub access_token: Option<String>,
    pub created_at: time::OffsetDateTime,
    pub updated_at: Option<time::OffsetDateTime>,
}

impl User {
    pub async fn get_user_by_github_id(
        db: &PgPool,
        github_id: i32,
    ) -> Result<Option<User>, AppError> {
        let db_user: Option<User> = sqlx::query_as!(
            crate::models::User,
            "select * from users where github_id = $1",
            github_id
        )
        .fetch_optional(db)
        .await?;
        Ok(db_user)
    }

    pub async fn create_user(
        db: &PgPool,
        github_id: i32,
        github_login: &str,
        access_token: &str,
    ) -> Result<User, AppError> {
        let new_user: User = sqlx::query_as("insert into users (github_id, github_login, access_token) values($1, $2, $3) returning *")
            .bind(github_id)
            .bind(github_login)
            .bind(access_token)
            .fetch_one(db).await?;
        Ok(new_user)
    }
}
