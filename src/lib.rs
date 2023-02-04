pub mod controller;
pub mod db;
pub mod error;
pub mod model;
pub mod utils;

// JWT secret key
static JWT_KEYS: once_cell::sync::Lazy<model::jwt::Keys> = once_cell::sync::Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your_secret".to_owned());
    model::jwt::Keys::new(secret.as_bytes())
});
