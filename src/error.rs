#[derive(Debug)]
pub enum AppError {
    InvalidToken,
    WrongEmailOrPassword,
    TokenCreationFailed,
    InternalServerError,
    EmailAlreadyExist,
    DatabaseInitFailed,
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, message) = match self {
            AppError::InvalidToken => (
                axum::http::StatusCode::BAD_REQUEST,
                "invalid_token",
                "invalid token",
            ),
            AppError::WrongEmailOrPassword => (
                axum::http::StatusCode::UNAUTHORIZED,
                "wrong_email_or_password",
                "wrong email or password",
            ),
            AppError::TokenCreationFailed => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "token_creation_failed",
                "token creation failed",
            ),
            AppError::InternalServerError => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "internal_server_error",
                "internal server error",
            ),
            AppError::EmailAlreadyExist => (
                axum::http::StatusCode::BAD_REQUEST,
                "email_already_exist",
                "email already exist",
            ),
            AppError::DatabaseInitFailed => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "database_init_failed",
                "database init failed",
            ),
        };
        (
            status,
            axum::Json(serde_json::json!({ "code": code, "message": message })),
        )
            .into_response()
    }
}
