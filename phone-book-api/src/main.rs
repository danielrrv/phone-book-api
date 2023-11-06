#[macro_use]
extern crate lazy_static;

use std::path::Path;
use std::env;
use lupa::{hello_lupa, connection::Connection};
use axum::{
    routing::{get, post},
    Router,
    http::StatusCode,
    response::{Response, Result, IntoResponse}
};


pub mod model;
pub mod handlers;
pub mod data;

use handlers::country_handler::handlers::{ get_country};
use mongodb::{options::{ClientOptions, ServerApiVersion, ServerApi}, Client};


use std::net::SocketAddr;


#[tokio::main]
async fn main() {
    let connection_path: &std::path::Path = Path::new("./config.toml");
    // initialize tracing
    tracing_subscriber::fmt::init();
    let mongo_client = Connection::from_config(connection_path)
        .get_client()
        .unwrap()
        .unwrap();

    // build our application with a route
    let app = Router::new()
        // .route("/", get(hello));
        .route("/:country", get(get_country))
        .with_state(mongo_client);        

    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    
    // tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
