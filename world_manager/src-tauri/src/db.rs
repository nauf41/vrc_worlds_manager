use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::{str::FromStr, sync::OnceLock};
pub mod discord;
pub mod log_files;
pub mod tag_groups;
pub mod tags;
pub mod worlds;

pub async fn get_pool() -> &'static sqlx::SqlitePool {
    static POOL: OnceLock<sqlx::SqlitePool> = OnceLock::new();

    if let Some(v) = POOL.get() {
        v
    } else {
        println!("[db.rs] accessing sqlite db...");
        let opts = SqliteConnectOptions::from_str("sqlite://test.db")
            .unwrap()
            .create_if_missing(true);
        let p = SqlitePool::connect_with(opts).await.unwrap();
        println!(
            "[db.rs] successfully connecteed to sqlite db. Updating reference and returning..."
        );
        POOL.get_or_init(|| p)
    }
}

pub async fn init() -> Result<(), sqlx::Error> {
    let pool = get_pool().await;

    // pragmas
    sqlx::query!("PRAGMA foreign_keys = ON;")
        .execute(pool)
        .await?;

    // create tables
    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS worlds (
      id INTEGER PRIMARY KEY,
      uuid TEXT UNIQUE NOT NULL,
      publisher_uuid TEXT,
      publisher_name TEXT,
      registered_at INTEGER,
      description TEXT,
      title TEXT,
      visits INTEGER,
      favorites INTEGER,
      capacity INTEGER,
      published_at INTEGER,
      does_support_windows INTEGER,
      does_support_android INTEGER,
      does_support_ios INTEGER,
      latest_at INTEGER, /* epoch msec */
      image_cache_file TEXT
    );"
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS tags (
      id INTEGER PRIMARY KEY,
      name TEXT NOT NULL
    );"
    )
    .execute(pool)
    .await?;

    sqlx::query!(r#"INSERT OR IGNORE INTO tags (id, name) VALUES (0, "__FAVORITED__");"#)
        .execute(pool)
        .await?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS tags_worlds (
      tags_id INTEGER NOT NULL,
      worlds_id INTEGER NOT NULL,
      is_sent_to_discord INTEGER NOT NULL,
      PRIMARY KEY (tags_id, worlds_id)
    );"
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS discord_channels (
      id INTEGER PRIMARY KEY,
      name TEXT NOT NULL
    );"
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS tags_discord_channels (
      tag_id INTEGER NOT NULL UNIQUE,
      discord_channel_id INTEGER NOT NULL,
      latest_read_id INTEGER,
      do_auto_fetch INTEGER NOT NULL,
      do_auto_post INTEGER NOT NULL,
      PRIMARY KEY (tag_id, discord_channel_id),
      FOREIGN KEY (tag_id) REFERENCES tags(id),
      FOREIGN KEY (discord_channel_id) REFERENCES discord_channels(id)
    );"
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS activities (
      id INTEGER PRIMARY KEY,
      world_id INTEGER,
      started_at INTEGER NOT NULL,
      ended_at INTEGER,
      FOREIGN KEY (world_id) REFERENCES worlds(id)
    );"
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        // for log watcher. name: output_log_xxx.txt, read_at: the byte offset of the log file that has been read.
        "CREATE TABLE IF NOT EXISTS log_files (
      id INTEGER PRIMARY KEY,
      name TEXT NOT NULL UNIQUE,
      read_at INTEGER NOT NULL
    );"
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS tag_groups (
      id INTEGER PRIMARY KEY,
      name TEXT NOT NULL
    );"
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS tag_groups_tags (
      tag_groups_id INTEGER NOT NULL,
      tags_id INTEGER NOT NULL UNIQUE,
      PRIMARY KEY (tag_groups_id, tags_id),
      FOREIGN KEY (tag_groups_id) REFERENCES tag_groups(id),
      FOREIGN KEY (tags_id) REFERENCES tags(id)
    );"
    )
    .execute(pool)
    .await?;

    std::fs::create_dir_all("./thumbnail-cache").unwrap();

    Ok(())
}
