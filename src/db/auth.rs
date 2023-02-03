pub async fn create(
    db_pool: &sqlx::SqlitePool,
    auth: &crate::model::auth::Auth,
) -> Result<i64, crate::error::AppError> {
    let result =
        sqlx::query("insert into auth (create_at, update_at, remark, user_id, email, password) values ($1, $2, $3, $4, $5, $6);")
            .bind(auth.create_at)
            .bind(auth.update_at)
            .bind(&auth.remark)
            .bind(auth.user_id)
            .bind(&auth.email)
            .bind(&auth.password)
            .execute(db_pool)
            .await
            .map_err(|error| {
                tracing::error!("{}", error);
                crate::error::AppError::InternalServerError
            })?;
    Ok(result.last_insert_rowid())
}

pub async fn get_by_email(
    db_pool: &sqlx::SqlitePool,
    email: &str,
) -> Result<Option<crate::model::auth::Auth>, crate::error::AppError> {
    sqlx::query_as::<_, crate::model::auth::Auth>(
        "select id, create_at, update_at, remark, user_id, email, password from auth where email = $1;",
    )
    .bind(email)
    .fetch_optional(db_pool)
    .await
    .map_err(|error| {
        tracing::error!("{}", error);
        crate::error::AppError::InternalServerError
    })
}
