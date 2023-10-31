mod errors;
mod middlewares;
mod routes;
pub mod types;

use std::{env, time::Duration};

use axum::{middleware, routing::get, Router};
use sqlx::postgres::{PgPoolOptions, Postgres};

use crate::web::middlewares::with_connection::{
    with_connection, with_transactioned_connection,
};

#[derive(Clone)]
pub struct PogloState {
    pool: sqlx::Pool<Postgres>,
}
impl PogloState {
    pub async fn new() -> Result<PogloState, anyhow::Error> {
        println!("acquiring pool...");

        let pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(3))
            .max_connections(1)
            .connect(&env::var("CONN_URI").unwrap())
            .await?;

        Ok(PogloState { pool })
    }
}

pub async fn build_app() -> Router {
    let state = PogloState::new().await.unwrap();
    println!("state ok");
    let app = Router::new()
        .route("/", get(routes::root::index))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            with_transactioned_connection,
        ))
        .with_state(state);
    app
}
