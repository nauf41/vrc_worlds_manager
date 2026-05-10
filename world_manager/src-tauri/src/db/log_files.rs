use crate::db::get_pool;

pub async fn get_log(log_name: &str) -> Result<Option<LogData>, sqlx::Error> {
    sqlx::query_as!(
        LogData,
        "
    SELECT
      name,
      read_at
    FROM log_files
    WHERE name = ?
    ;
    ",
        log_name
    )
    .fetch_optional(get_pool().await)
    .await
}

pub async fn new_log(log_name: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "
    INSERT INTO log_files (name, read_at)
    VALUES (?, 0)
    ;
    ",
        log_name
    )
    .execute(get_pool().await)
    .await?;

    Ok(())
}

pub async fn update_log_read_at(log_name: &str, read_at: i64) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "
    UPDATE log_files
    SET read_at = ?
    WHERE name = ?
    ;
    ",
        read_at,
        log_name
    )
    .execute(get_pool().await)
    .await?;

    Ok(())
}

pub struct LogData {
    pub name: String,
    pub read_at: i64,
}
