mod routes;
mod entity;
pub mod response;
mod middleware;

use crate::state::AppState;

pub async fn init(){
    // Init Routes
    let state = AppState::new();
    let routes = routes::init().with_state(state).into_make_service();
    // Init Axum
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on: 0.0.0.0:8080");
    axum::serve(listener, routes).await.unwrap()
}