pub async fn init_db() -> Result<sqlx::SqlitePool, crate::error::AppError> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&database_url)
        .await
    {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("{}", e);
            return Err(crate::error::AppError::DatabaseInitFailed);
        }
    };

    // auth table
    if let Err(e) = sqlx::query(
        "create table if not exists auth (
             id integer primary key autoincrement,
             create_at integer not null,
             update_at integer not null,
             remark text,
             user_id integer not null,
             email text not null,
             password text not null
         );",
    )
    .execute(&pool)
    .await
    {
        tracing::error!("{}", e);
        return Err(crate::error::AppError::DatabaseInitFailed);
    };

    // user table
    if let Err(e) = sqlx::query(
        "create table if not exists user (
             id integer primary key autoincrement,
             create_at integer not null,
             update_at integer not null,
             remark text
         );",
    )
    .execute(&pool)
    .await
    {
        tracing::error!("{}", e);
        return Err(crate::error::AppError::DatabaseInitFailed);
    };

    // link table
    if let Err(e) = sqlx::query(
        "create table if not exists link (
             id text primary key,
             create_at integer not null,
             update_at integer not null,
             remark text,
             user_id integer not null,
             target_link text not null,
             visits_count integer not null
         );",
    )
    .execute(&pool)
    .await
    {
        tracing::error!("{}", e);
        return Err(crate::error::AppError::DatabaseInitFailed);
    };

    Ok(pool)
}
