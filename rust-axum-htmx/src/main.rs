use anyhow::Context;
use html::AppState;
use minijinja::{path_loader, Environment};
use minijinja_autoreload::AutoReloader;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_sessions::PostgresStore;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

mod error;
mod html;
mod models;

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

    let reloader = AutoReloader::new(move |notfier| {
        let template_path = "./templates";
        let mut env = Environment::new();
        env.set_loader(path_loader(template_path));

        notfier.set_fast_reload(true);
        notfier.watch_path(template_path, true);

        Ok(env)
    });
    let reloader = Arc::new(reloader);

    let oauth_client = html::oauth::oauth_client().unwrap();

    let app_state = AppState {
        db,
        reloader,
        oauth_client,
    };
    html::serve(app_state).await
}
