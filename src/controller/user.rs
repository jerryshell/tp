pub async fn profile(
    axum::Extension(db_pool): axum::Extension<sqlx::SqlitePool>,
    header_map: axum::http::HeaderMap,
) -> Result<axum::Json<serde_json::Value>, crate::error::AppError> {
    let calims = crate::model::jwt::Calims::from_request_header_map(header_map)?;
    let user_id = calims.user_id;
    let user = crate::db::user::get_by_id(&db_pool, &user_id).await?;
    match user {
        None => Err(crate::error::AppError::UserDoesNotExist),
        Some(user) => Ok(axum::Json(serde_json::json!({
            "code": "success",
            "userId": user_id,
            "createAt": user.create_at,
            "updateAt": user.update_at,
        }))),
    }
}
