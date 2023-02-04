#[tokio::main]
async fn main() -> Result<(), tp::error::AppError> {
    // init env variable
    dotenvy::dotenv().ok();

    // init database
    let db_pool = tp::db::init::init_db().await?;

    // init tracing
    tracing_subscriber::fmt::init();

    // build app with route
    let app = axum::Router::new()
        .route("/", axum::routing::get(tp::controller::index::index))
        .route(
            "/auth/register",
            axum::routing::post(tp::controller::auth::register),
        )
        .route(
            "/auth/login",
            axum::routing::post(tp::controller::auth::login),
        )
        .route(
            "/user/profile",
            axum::routing::get(tp::controller::user::profile),
        )
        .layer(axum::Extension(db_pool));

    // run app
    let port = std::env::var("PORT")
        .expect("PORT must be set")
        .parse::<u16>()
        .expect("PORT must be unsigned number");
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
