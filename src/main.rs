use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        backend_apple::setup::setup_frames();
        println!("video is setup");
    });

    let app = Router::new().route("/frames/{frame_id}", get(backend_apple::handlers::frame));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
