pub async fn register(
    axum::Extension(db_pool): axum::Extension<sqlx::SqlitePool>,
    axum::Json(form): axum::Json<crate::model::auth::RegisterForm>,
) -> Result<axum::Json<serde_json::Value>, crate::error::AppError> {
    // check if email or password is blank
    if form.email.is_empty() || form.password.is_empty() {
        return Err(crate::error::AppError::WrongEmailOrPassword);
    }

    // get auth by email from database
    let auth_by_email = crate::db::auth::get_by_email(&db_pool, &form.email).await?;

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
        email: form.email,
        password: form.password,
    };

    // insert auth into database
    let _auth_id = crate::db::auth::create(&db_pool, &auth).await?;

    Ok(axum::Json(serde_json::json!({ "code": "success" })))
}

pub async fn login(
    axum::Extension(db_pool): axum::Extension<sqlx::SqlitePool>,
    axum::Json(form): axum::Json<crate::model::auth::LoginForm>,
) -> Result<axum::Json<serde_json::Value>, crate::error::AppError> {
    // check if email or password is blank
    if form.email.is_empty() || form.password.is_empty() {
        return Err(crate::error::AppError::WrongEmailOrPassword);
    }

    // get auth by email from database
    let auth_by_email = crate::db::auth::get_by_email(&db_pool, &form.email).await?;

    match auth_by_email {
        // if auth_by_email does not exist, send error
        None => Err(crate::error::AppError::WrongEmailOrPassword),
        Some(auth_by_email) => {
            // check password
            if auth_by_email.password != form.password {
                return Err(crate::error::AppError::WrongEmailOrPassword);
            }

            // get user by auth_by_email.user_id
            let user = crate::db::user::get_by_id(&db_pool, &auth_by_email.user_id).await?;

            match user {
                // if user does not exist, send error
                None => Err(crate::error::AppError::UserDoesNotExist),
                // create jwt
                Some(user) => {
                    let claims = crate::model::jwt::Calims {
                        user_id: user.id,
                        exp: crate::utils::get_timestamp_n_hours_from_now(8),
                    };
                    let token = jsonwebtoken::encode(
                        &jsonwebtoken::Header::default(),
                        &claims,
                        &crate::JWT_KEYS.encoding,
                    )
                    .map_err(|_| crate::error::AppError::TokenCreationFailed)?;
                    Ok(axum::Json(serde_json::json!({
                        "code": "success",
                        "token": token,
                        "userId": user.id,
                    })))
                }
            }
        }
    }
}

pub async fn update_email(
    axum::Extension(db_pool): axum::Extension<sqlx::SqlitePool>,
    header_map: axum::http::HeaderMap,
    axum::Json(form): axum::Json<crate::model::auth::UpdateEmailForm>,
) -> Result<axum::Json<serde_json::Value>, crate::error::AppError> {
    // get clims from header map
    let calims = crate::model::jwt::Calims::from_request_header_map(header_map)?;

    // check if email is blank
    if form.email.is_empty() {
        return Err(crate::error::AppError::FailedWithMessage("email is blank"));
    }

    // get auth by id
    let auth = crate::db::auth::get_by_id(&db_pool, form.id).await?;

    match auth {
        // check if auth is none
        None => Err(crate::error::AppError::FailedWithMessage(
            "auth does not exist",
        )),
        Some(mut auth) => {
            // check auth.user_id == calims.user_id
            if auth.user_id != calims.user_id {
                return Err(crate::error::AppError::FailedWithMessage("no permissions"));
            }

            // update email
            let now = crate::utils::get_timestamp_n_hours_from_now(0);
            auth.update_at = now as u32;
            auth.email = form.email;
            crate::db::auth::update_email(&db_pool, &auth).await?;

            Ok(axum::Json(serde_json::json!({
               "code": "success",
               "email": auth.email,
            })))
        }
    }
}

pub async fn update_password(
    axum::Extension(db_pool): axum::Extension<sqlx::SqlitePool>,
    header_map: axum::http::HeaderMap,
    axum::Json(form): axum::Json<crate::model::auth::UpdatePasswordForm>,
) -> Result<axum::Json<serde_json::Value>, crate::error::AppError> {
    // get clims from header map
    let calims = crate::model::jwt::Calims::from_request_header_map(header_map)?;

    // check if password is blank
    if form.password.is_empty() {
        return Err(crate::error::AppError::FailedWithMessage(
            "password is blank",
        ));
    }

    // get auth by id
    let auth = crate::db::auth::get_by_id(&db_pool, form.id).await?;

    match auth {
        // check if auth is none
        None => Err(crate::error::AppError::FailedWithMessage(
            "auth does not exist",
        )),
        Some(mut auth) => {
            // check auth.user_id == calims.user_id
            if auth.user_id != calims.user_id {
                return Err(crate::error::AppError::FailedWithMessage("no permissions"));
            }

            // update password
            let now = crate::utils::get_timestamp_n_hours_from_now(0);
            auth.update_at = now as u32;
            auth.password = form.password;
            crate::db::auth::update_password(&db_pool, &auth).await?;

            Ok(axum::Json(serde_json::json!({
               "code": "success",
               "password": auth.password,
            })))
        }
    }
}
