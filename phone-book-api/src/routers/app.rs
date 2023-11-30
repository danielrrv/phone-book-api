

use axum::{
    routing::{get, post, put},
    Router,
};

use lupa::connection::Connection;
use std::path::Path;
use crate::api::{country::get_country, bussiness::{get_businesses, put_business}};




pub fn application() -> Router {
    let connection_path: &std::path::Path = Path::new("./config.toml");
    // initialize tracing
    tracing_subscriber::fmt::init();
    let mongo_client = Connection::from_config(connection_path)
        .get_client()
        .unwrap()
        .unwrap();

    // build our application with a route
    return Router::new()
        // .route("/", get(hello));
        .route("/:country", get(get_country))
        .route("/:country/:city", get(get_businesses))
        .route("/:country", put(put_business))    
        .with_state(mongo_client);
}
