use anyhow::Context;
use minijinja::{path_loader, Environment};
use minijinja_autoreload::AutoReloader;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

mod html;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    reloader: Arc<AutoReloader>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let reloader = AutoReloader::new(move |notfier| {
        let template_path = "./templates";
        let mut env = Environment::new();
        env.set_loader(path_loader(template_path));

        notfier.set_fast_reload(true);
        notfier.watch_path(template_path, true);

        Ok(env)
    });
    let reloader = Arc::new(reloader);

    let app_state = AppState { db, reloader };
    html::serve(app_state).await
}
