pub async fn create(
    db_pool: &sqlx::SqlitePool,
    link: &crate::model::link::Link,
) -> Result<i64, crate::error::AppError> {
    let result =
        sqlx::query("insert into link (id, create_at, update_at, remark, user_id, target_link, visits_count) values ($1, $2, $3, $4, $5, $6, $7);")
            .bind(&link.id)
            .bind(link.create_at)
            .bind(link.update_at)
            .bind(&link.remark)
            .bind(link.user_id)
            .bind(&link.target_link)
            .bind(link.visits_count)
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
    id: &str,
) -> Result<Option<crate::model::link::Link>, crate::error::AppError> {
    sqlx::query_as::<_, crate::model::link::Link>(
        "select id, create_at, update_at, remark, user_id, target_link, visits_count from link where id = $1;",
    )
    .bind(id)
    .fetch_optional(db_pool)
    .await
    .map_err(|error| {
        tracing::error!("{}", error);
        crate::error::AppError::InternalServerError
    })
}
