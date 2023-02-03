pub async fn index() -> Result<axum::Json<serde_json::Value>, crate::error::AppError> {
    Ok(axum::Json(serde_json::json!({
        "repository": "https://github.com/jerryshell/tp",
        "license": "https://choosealicense.com/licenses/agpl-3.0"
    })))
}
