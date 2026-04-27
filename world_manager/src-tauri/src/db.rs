use std::{str::FromStr, sync::OnceLock};
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

pub async fn get_pool() -> &'static sqlx::SqlitePool {
  static POOL: OnceLock<sqlx::SqlitePool> = OnceLock::new();

  if let Some(v) = POOL.get() {
    v
  } else {
    println!("[db.rs] accessing sqlite db...");
    let opts = SqliteConnectOptions::from_str("sqlite://test.db").unwrap()
      .create_if_missing(true);
    let p = SqlitePool::connect_with(opts).await.unwrap();
    println!("[db.rs] successfully connecteed to sqlite db. Updating reference and returning...");
    POOL.get_or_init(|| p)
  }
}

pub async fn init() -> anyhow::Result<()> {
  let pool = get_pool().await;

  // pragmas
  sqlx::query!(
    "PRAGMA foreign_keys = ON;"
  ).execute(pool).await?;

  // create tables
  sqlx::query!(
    "CREATE TABLE IF NOT EXISTS worlds (
      id INTEGER PRIMARY KEY,
      uuid TEXT UNIQUE NOT NULL,
      publisher INTEGER NOT NULL,
      FOREIGN KEY (publisher) REFERENCES users(id)
    );"
  ).execute(pool).await?;

  sqlx::query!(
    "CREATE TABLE IF NOT EXISTS users (
      id INTEGER PRIMARY KEY,
      uuid TEXT NOT NULL
    );"
  ).execute(pool).await?;

  sqlx::query!(
    "CREATE TABLE IF NOT EXISTS worlds_cache (
      id INTEGER PRIMARY KEY,
      world_id INTEGER NOT NULL,
      cached_at INTEGER NOT NULL,
      description TEXT NOT NULL,
      title TEXT NOT NULL,
      visits INTEGER,
      favorites INTEGER,
      capacity INTEGER,
      published_at INTEGER,
      does_support_windows INTEGER,
      does_support_android INTEGER,
      does_support_ios INTEGER,
      FOREIGN KEY (world_id) REFERENCES worlds(id)
    );"
  ).execute(pool).await?;

  sqlx::query!(
    "CREATE TABLE IF NOT EXISTS tags (
      id INTEGER PRIMARY KEY,
      name TEXT NOT NULL
    );"
  ).execute(pool).await?;

  sqlx::query!(
    "CREATE TABLE IF NOT EXISTS tags_worlds (
      tags_id INTEGER NOT NULL,
      worlds_id INTEGER NOT NULL,
      is_sent_to_discord INTEGER NOT NULL,
      PRIMARY KEY (tags_id, worlds_id)
    );"
  ).execute(pool).await?;

  sqlx::query!(
    "CREATE TABLE IF NOT EXISTS folders (
      id INTEGER PRIMARY KEY,
      name TEXT NOT NULL
    );"
  ).execute(pool).await?;

  sqlx::query!(
    "CREATE TABLE IF NOT EXISTS discord_channels (
      id INTEGER PRIMARY KEY,
      uuid TEXT NOT NULL,
      name TEXT NOT NULL,
      latest_read_snowflake TEXT
    );"
  ).execute(pool).await?;

  sqlx::query!(
    "CREATE TABLE IF NOT EXISTS folders_discord_channels (
      folder_id INTEGER NOT NULL,
      discord_channel_id INTEGER NOT NULL,
      do_auto_fetch INTEGER NOT NULL,
      do_auto_post INTEGER NOT NULL
    );"
  ).execute(pool).await?;

  sqlx::query!(
    "CREATE TABLE IF NOT EXISTS activities (
      id INTEGER PRIMARY KEY,
      world_id INTEGER,
      started_at INTEGER NOT NULL,
      ended_at INTEGER,
      FOREIGN KEY (world_id) REFERENCES world(id)
    );"
  ).execute(pool).await?;

  sqlx::query!(
    "CREATE TABLE IF NOT EXISTS users_cache (
      id INTEGER PRIMARY KEY,
      user_id INTEGER,
      name TEXT NOT NULL,
      FOREIGN KEY (user_id) REFERENCES user(id)
    );"
  ).execute(pool).await?;

  Ok(())
}

pub async fn add_new_world(uuid: String, publisher: i32) -> anyhow::Result<()> {
  sqlx::query!(
    "INSERT INTO worlds (uuid, publisher) VALUES (
      ?,
      ?
    );",
    uuid,
    publisher
  ).execute(get_pool().await).await?;

  Ok(())
}

pub async fn does_world_exist(uuid: String) -> anyhow::Result<bool> {
  let len: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM worlds WHERE uuid = ?;", uuid)
    .fetch_one(get_pool().await)
    .await?;

  Ok(len > 0)
}

pub async fn add_new_user(uuid: String) -> anyhow::Result<()> {
  sqlx::query!(
    "INSERT INTO users (uuid) VALUES (
      ?
    );",
    uuid
  ).execute(get_pool().await).await?;

  Ok(())
}
