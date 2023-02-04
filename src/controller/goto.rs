pub async fn goto(
    axum::Extension(db_pool): axum::Extension<sqlx::SqlitePool>,
    axum::extract::Path(link_id): axum::extract::Path<String>,
) -> crate::model::goto::GotoResponse {
    match crate::db::link::get_by_id(&db_pool, &link_id).await {
        Err(e) => crate::model::goto::GotoResponse::AppError(e),
        Ok(link) => match link {
            None => crate::model::goto::GotoResponse::AppError(
                crate::error::AppError::FailedWithMessage("link not found"),
            ),
            Some(mut link) => {
                link.update_at = crate::utils::get_timestamp_n_hours_from_now(0) as u32;
                link.visits_count += 1;
                let _result = crate::db::link::update_visits_count(&db_pool, &link).await;
                crate::model::goto::GotoResponse::Redirect(link.target_link.clone())
            }
        },
    }
}
