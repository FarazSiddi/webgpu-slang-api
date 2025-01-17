use axum::{
    routing::{get},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// A simple data struct we might return as JSON
#[derive(Serialize, Deserialize)]
struct ExampleResponse {
    message: String,
}

#[tokio::main]
async fn main() {
    // Build our application with a single route
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/hello", get(hello_handler));

    // Specify the address to serve on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Listening on {}", addr);

    // Run our app using hyper
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler for the root route
async fn root_handler() -> &'static str {
    "Hello from Rust backend!"
}

// Handler for /hello route
async fn hello_handler() -> Json<ExampleResponse> {
    Json(ExampleResponse {
        message: "Hello from JSON endpoint".to_string(),
    })
}
