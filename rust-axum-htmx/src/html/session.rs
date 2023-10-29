use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use time::Duration;
use tower_sessions::{PostgresStore, SessionManagerLayer};

pub fn service(db: Pool<Postgres>) -> SessionManagerLayer<PostgresStore> {
    let session_store = PostgresStore::new(db.clone());

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_max_age(Duration::days(7));
    session_layer
}

#[derive(Serialize, Deserialize, Default)]
pub struct GithubAccessToken(pub String);
impl GithubAccessToken {
    pub fn key() -> &'static str {
        "GithubAccessToken"
    }
}
