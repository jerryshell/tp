pub async fn create(
    db_pool: &sqlx::SqlitePool,
    user: &crate::model::user::User,
) -> Result<i64, crate::error::AppError> {
    let result =
        sqlx::query("insert into user (create_at, update_at, remark) values ($1, $2, $3);")
            .bind(user.create_at)
            .bind(user.update_at)
            .bind(&user.remark)
            .execute(db_pool)
            .await
            .map_err(|error| {
                tracing::error!("{}", error);
                crate::error::AppError::InternalServerError
            })?;
    Ok(result.last_insert_rowid())
}

pub async fn get_by_id(
    db_pool: &sqlx::SqlitePool,
    id: &u32,
) -> Result<Option<crate::model::user::User>, crate::error::AppError> {
    sqlx::query_as::<_, crate::model::user::User>(
        "select id, create_at, update_at, remark from user where id = $1;",
    )
    .bind(id)
    .fetch_optional(db_pool)
    .await
    .map_err(|error| {
        tracing::error!("{}", error);
        crate::error::AppError::InternalServerError
    })
}
