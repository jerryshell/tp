pub async fn create(
    axum::Extension(db_pool): axum::Extension<sqlx::SqlitePool>,
    header_map: axum::http::HeaderMap,
    axum::Json(create_form): axum::Json<crate::model::link::CreateForm>,
) -> Result<axum::Json<serde_json::Value>, crate::error::AppError> {
    // get clims from header map
    let calims = crate::model::jwt::Calims::from_request_header_map(header_map)?;

    // check id or target_link is blank
    if create_form.id.is_empty() || create_form.target_link.is_empty() {
        return Err(crate::error::AppError::LinkIdOrTargetLinkIsBlank);
    }

    // get link by id from database
    let link_by_id = crate::db::link::get_by_id(&db_pool, &create_form.id).await?;

    // if link_by_id already exist, send error
    if link_by_id.is_some() {
        return Err(crate::error::AppError::FailedWithMessage(
            "link_id already exist",
        ));
    }

    // init link struct
    let now = crate::utils::get_timestamp_n_hours_from_now(0);
    let link = crate::model::link::Link {
        id: create_form.id,
        create_at: now as u32,
        update_at: now as u32,
        remark: "".to_string(),
        user_id: calims.user_id,
        target_link: create_form.target_link,
        visits_count: 0,
    };

    // insert link into database
    let _link_id = crate::db::link::create(&db_pool, &link).await?;

    Ok(axum::Json(serde_json::json!({
       "code": "success",
       "id": link.id,
       "targetLink": link.target_link,
    })))
}
