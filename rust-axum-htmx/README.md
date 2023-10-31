# Rust-Axum-Htmx

+ Axum as Web Framework
+ Sqlx with postgresql for the database
+ aksama and maud for templeting
+ a bit of htmx for interactivity

## Setup

1. Copy `.env.sample` to `.env` make sure that you have postgres running and the `DATABASE_URL` is correct.
2. Create an oauth app at: https://github.com/settings/developers and put the client id and secret into `.env`  
    2.1. Github requires that the `User-Agent` header is set to your username or the name off the previously created app, see [the docs](https://docs.github.com/en/rest/overview/resources-in-the-rest-api?apiVersion=2022-11-28#user-agent-required).
    Change the name used in the User-Agent by running:  
    `sed -i 's/oauth_test/<name>/' src/github.rs`
3. `sqlx database setup` (you may need to install sqlx first)
4. `cargo run`
