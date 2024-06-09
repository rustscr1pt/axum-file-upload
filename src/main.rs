mod routes;

use axum::Router;
use crate::routes::user_routes::user_routes;

#[tokio::main]
async fn main() {
    let app : Router = Router::new()
        .merge(user_routes());

    let addr = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", 8000)).await.unwrap();
    println!("Running on http://localhost:{}", 8000);
    axum::serve(addr, app).await.unwrap();
}
