use backend::routing;

//don't unwrap in prod

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = routing::app().await;

    println!("listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}