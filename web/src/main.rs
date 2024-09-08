use axum::Server;
use std::net::SocketAddr;
use std::sync::Arc;

mod router;

#[derive(Clone)]
pub struct AppState {
    // Add any shared state here
}

#[tokio::main]
async fn main() {
    // Initialize your AppState here
    let state = AppState {
        // Initialize your state
    };

    let app = router::create_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
