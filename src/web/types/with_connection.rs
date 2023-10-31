use std::sync::Arc;

use sqlx::{pool, Postgres};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct WithConnection(pub Arc<Mutex<pool::PoolConnection<Postgres>>>);
