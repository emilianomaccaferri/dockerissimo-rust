mod routes;
use axum::{routing::get, Router};

pub fn build_app() -> Router {
    let app = Router::new().route("/", get(routes::root::index));

    app
}
