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

pub async fn get_by_id(
    db_pool: &sqlx::SqlitePool,
    id: u32,
) -> Result<Option<crate::model::auth::Auth>, crate::error::AppError> {
    sqlx::query_as::<_, crate::model::auth::Auth>(
        "select id, create_at, update_at, remark, user_id, email, password from auth where id = $1;",
    )
    .bind(id)
    .fetch_optional(db_pool)
    .await
    .map_err(|error| {
        tracing::error!("{}", error);
        crate::error::AppError::InternalServerError
    })
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

pub async fn update_email(
    db_pool: &sqlx::SqlitePool,
    auth: &crate::model::auth::Auth,
) -> Result<(), crate::error::AppError> {
    let _result = sqlx::query("update auth set update_at=$1, email=$2 where id=$3")
        .bind(auth.update_at)
        .bind(&auth.email)
        .bind(auth.id)
        .execute(db_pool)
        .await
        .map_err(|error| {
            tracing::error!("{}", error);
            crate::error::AppError::InternalServerError
        })?;
    Ok(())
}

pub async fn update_password(
    db_pool: &sqlx::SqlitePool,
    auth: &crate::model::auth::Auth,
) -> Result<(), crate::error::AppError> {
    let _result = sqlx::query("update auth set update_at=$1, password=$2 where id=$3")
        .bind(auth.update_at)
        .bind(&auth.password)
        .bind(auth.id)
        .execute(db_pool)
        .await
        .map_err(|error| {
            tracing::error!("{}", error);
            crate::error::AppError::InternalServerError
        })?;
    Ok(())
}
