mod cli;
mod log_util;
mod web;

use std::net::SocketAddr;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() {
    let parse = Cli::parse();
    println!("conf-path: {:?}", parse.conf_path());
    println!("templates-path: {:?}", parse.templates_path());
    let router = web::build_app().await;
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
