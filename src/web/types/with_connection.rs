use std::sync::Arc;
use tokio::sync::Mutex;

use sqlx::{pool, Postgres};

#[derive(Clone)]
pub struct WithConnection(pub Arc<Mutex<pool::PoolConnection<Postgres>>>);
