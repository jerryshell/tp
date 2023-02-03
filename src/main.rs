#[tokio::main]
async fn main() -> Result<(), tp::error::AppError> {
    // init env variable
    dotenvy::dotenv().ok();

    // init database
    tp::init_sqlite_db().await?;

    // init tracing
    tracing_subscriber::fmt::init();

    // build app with route
    let app = axum::Router::new().route("/", axum::routing::get(tp::controller::index::index));

    // run app
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
