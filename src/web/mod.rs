mod routes;
pub mod extractors;
mod errors;
mod middlewares;

use std::{time::Duration, env};

use axum::{routing::get, Router};
use sqlx::postgres::{Postgres, PgPoolOptions};

#[derive(Clone)]
struct PogloState {
    pool: sqlx::Pool<Postgres>,
}
impl PogloState {
    pub async fn new() -> Result<PogloState, anyhow::Error> {
        
        println!("acquiring pool...");

        let pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(3))
            .max_connections(5)
            .connect(&env::var("CONN_URI").unwrap()).await?;

        Ok(PogloState { pool })
    }
}

pub async fn build_app() -> Router {
    let state = PogloState::new().await.unwrap();
    println!("state ok");
    let app = 
        Router::new()
            .route("/", get(routes::root::index))
            .with_state(state);
    app
}
