pub enum GotoResponse {
    AppError(crate::error::AppError),
    Redirect(String),
}

impl axum::response::IntoResponse for GotoResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            GotoResponse::AppError(e) => e.into_response(),
            GotoResponse::Redirect(target_link) => {
                axum::response::Redirect::to(&target_link).into_response()
            }
        }
    }
}
