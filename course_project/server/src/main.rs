use axum::{
	routing::get,
	Router
};

#[tokio::main]
async fn main() {
	let app = Router::new().route("/", get(|| async { "hello world!"}));

	let listener = tokio::net::TcpListener::bind("localhost:3001").await.unwrap();
	axum::serve(listener, app).await.unwrap()
}
