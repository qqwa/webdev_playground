use anyhow::Context;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use minijinja::{context, path_loader, Environment};
use minijinja_autoreload::AutoReloader;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
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

    info!("Hello World");

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

    let router = Router::new().route("/", get(hello)).with_state(app_state);
    let port = 4000u16;
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

#[derive(Debug)]
struct User2 {
    username: String,
}

async fn hello(State(state): State<AppState>) -> impl IntoResponse {
    let user = sqlx::query_as!(User2, "SELECT username from users")
        .fetch_one(&state.db)
        .await
        .unwrap();
    info!("{:?}", user);

    let env = state.reloader.acquire_env().unwrap();
    let template = env.get_template("index.html").unwrap();
    let username = user.username;
    let render = template.render(context! {username}).unwrap();
    Html(render)
}
