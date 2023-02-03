#[tokio::main]
async fn main() {
    // init tracing
    tracing_subscriber::fmt::init();

    // build app with route
    let app = axum::Router::new().route("/", axum::routing::get(|| async { "Hello, world" }));

    // run app
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
