use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use time::Duration;
use tower_sessions::{cookie::SameSite, PostgresStore, SessionManagerLayer};

pub fn service(db: Pool<Postgres>) -> SessionManagerLayer<PostgresStore> {
    let session_store = PostgresStore::new(db.clone());

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_max_age(Duration::days(7))
        .with_same_site(SameSite::Lax);
    session_layer
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Counter(pub u32);
impl Counter {
    pub fn key() -> &'static str {
        "Count"
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct GithubAccessToken(pub String);
impl GithubAccessToken {
    pub fn key() -> &'static str {
        "GithubAccessToken"
    }
}
