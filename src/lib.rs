pub async fn init_sqlite_db() -> Result<sqlx::Pool<sqlx::sqlite::Sqlite>, String> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&database_url)
        .await
    {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };

    // auth table
    if let Err(e) = sqlx::query(
        "create table if not exists auth (
             id integer primary key autoincrement,
             create_at integer not null,
             update_at integer not null,
             remark text,
             email text not null,
             password text not null,
             user_id integer not null
         )",
    )
    .execute(&pool)
    .await
    {
        return Err(e.to_string());
    };

    // user table
    if let Err(e) = sqlx::query(
        "create table if not exists user (
             id integer primary key autoincrement,
             create_at integer not null,
             update_at integer not null,
             remark text
         )",
    )
    .execute(&pool)
    .await
    {
        return Err(e.to_string());
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
         )",
    )
    .execute(&pool)
    .await
    {
        return Err(e.to_string());
    };

    Ok(pool)
}
