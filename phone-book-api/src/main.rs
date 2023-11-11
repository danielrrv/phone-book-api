
pub mod model;
pub mod api;
pub mod routers;

use routers::app::application;
use std::net::SocketAddr;


#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(application().into_make_service())
        .await
        .unwrap();
}
