pub async fn register(
    axum::Extension(db_pool): axum::Extension<sqlx::SqlitePool>,
    axum::Json(register_form): axum::Json<crate::model::auth::RegisterForm>,
) -> Result<axum::Json<serde_json::Value>, crate::error::AppError> {
    // check if email or password is blank
    if register_form.email.is_empty() || register_form.password.is_empty() {
        return Err(crate::error::AppError::WrongEmailOrPassword);
    }

    // get auth by email from database
    let auth_by_email = crate::db::auth::get_by_email(&db_pool, &register_form.email).await?;

    // if auth_by_email already exist, send error
    if auth_by_email.is_some() {
        return Err(crate::error::AppError::EmailAlreadyExist);
    }

    // init user struct
    let now = crate::utils::get_timestamp_n_hours_from_now(0);
    let user = crate::model::user::User {
        id: 0,
        create_at: now as u32,
        update_at: now as u32,
        remark: "".to_string(),
    };

    // insert user into database
    let user_id = crate::db::user::create(&db_pool, &user).await?;

    // init auth struct
    let auth = crate::model::auth::Auth {
        id: 0,
        create_at: now as u32,
        update_at: now as u32,
        remark: "".to_string(),
        user_id: user_id as u32,
        email: register_form.email,
        password: register_form.password,
    };

    // insert auth into database
    let _auth_id = crate::db::auth::create(&db_pool, &auth).await?;

    Ok(axum::Json(serde_json::json!({ "code": "success" })))
}
