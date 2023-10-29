use anyhow::Context;
use html::AppState;
use sqlx::postgres::PgPoolOptions;
use tower_sessions::PostgresStore;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

mod error;
mod html;
mod models;
mod views;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_axum_htmx=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&db_url)
        .await
        .context("could not connect to database_url")?;
    sqlx::migrate!().run(&db).await?;
    PostgresStore::new(db.clone()).migrate().await?;

    let oauth_client = html::oauth::oauth_client().unwrap();

    let app_state = AppState { db, oauth_client };
    html::serve(app_state).await
}
