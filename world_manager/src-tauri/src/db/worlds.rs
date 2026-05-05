use serde::{Deserialize, Serialize};

use super::get_pool;

fn boolo_to_i64o(b: &Option<bool>) -> Option<i64> {
  b.map(|v| if v { 1 } else { 0 })
}

/// just add worlds to the DB, not register
pub async fn add_new_world(uuid: &str, publisher: Option<i64>) -> Result<(), sqlx::Error> {
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

pub async fn add_world_cache(world: &crate::ipc::native_messaging::World, cache: &crate::ipc::native_messaging::WorldCache) -> Result<(), sqlx::Error> {
  let now = chrono::Utc::now().timestamp_millis();
  let does_support_windows = boolo_to_i64o(&cache.does_support_windows);
  let does_support_android = boolo_to_i64o(&cache.does_support_android);
  let does_support_ios = boolo_to_i64o(&cache.does_support_ios);
  sqlx::query!(
    "INSERT INTO worlds_cache (
      world_id,
      cached_at,
      description,
      title,
      visits,
      favorites,
      capacity,
      published_at,
      does_support_windows,
      does_support_android,
      does_support_ios
    ) VALUES (
      (SELECT id FROM worlds WHERE uuid = ?),
      ?,
      ?,
      ?,
      ?,
      ?,
      ?,
      ?,
      ?,
      ?,
      ?
    );",
    world.uuid,
    now,
    cache.description,
    cache.title,
    cache.visits,
    cache.favorites,
    cache.capacity,
    cache.published_at,
    does_support_windows,
    does_support_android,
    does_support_ios
  ).execute(get_pool().await).await?;

  Ok(())
}

/// just add worlds to the DB, not register
pub async fn add_new_world_if_not_exists(uuid: &str) -> Result<(), sqlx::Error> {
  sqlx::query!(
    "INSERT OR IGNORE INTO worlds (uuid) VALUES (
      ?
    );",
    uuid
  ).execute(get_pool().await).await?;

  Ok(())
}

pub async fn upsert_publisher(uuid: &str, name: &Option<String>) -> Result<(), sqlx::Error> {
  sqlx::query!(
    "INSERT OR IGNORE INTO users (uuid) VALUES (?) RETURNING id;",
    uuid
  ).execute(get_pool().await).await?;

  let user_id = data.id;

  if name.is_some() {
    let now = chrono::Utc::now().timestamp_millis();
    sqlx::query!(
      "INSERT INTO users_cache (user_id, cached_at, name) VALUES (
        ?,
        ?,
        ?
      );",
      user_id,
      now,
      name
    ).execute(get_pool().await).await?;
  }

  Ok(())
}

pub async fn does_world_exist(uuid: &str) -> Result<bool, sqlx::Error> {
  let len: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM worlds WHERE uuid = ?;", uuid)
    .fetch_one(get_pool().await)
    .await?;

  Ok(len > 0)
}

pub async fn get_world_id_by_uuid(uuid: &str) -> Result<Option<i64>, sqlx::Error> {
  let id = sqlx::query_scalar!("SELECT id FROM worlds WHERE uuid = ?;", uuid)
    .fetch_optional(get_pool().await)
    .await?;

  if let Some(Some(id)) = id {
    Ok(Some(id))
  } else {
    Ok(None)
  }
}

pub async fn get_worlds(filter: &WorldQueryFilters, _sort_by: &SortBy) -> Result<Vec<World>, sqlx::Error> {
  let res: Vec<sql_return_types::World> = sqlx::query_as!(
    sql_return_types::World,
    "
    SELECT
      worlds.id AS id,
      worlds.uuid AS uuid,
      worlds.publisher AS publisher,
      users_cache_1.name AS publisher_name,
      worlds_cache_1.description AS description,
      worlds_cache_1.title AS title,
      worlds_cache_1.visits AS visits,
      worlds_cache_1.favorites AS favorites,
      worlds_cache_1.capacity AS capacity,
      worlds_cache_1.published_at as published_at,
      worlds_cache_1.does_support_windows as does_support_windows,
      worlds_cache_1.does_support_android as does_support_android,
      worlds_cache_1.does_support_ios as does_support_ios,
      activities_1.self_visits as self_visits
    FROM worlds
    LEFT JOIN (
        SELECT *,
          ROW_NUMBER() OVER (PARTITION BY users_cache.user_id ORDER BY id DESC) AS rn
        FROM users_cache
      ) users_cache_1
      ON worlds.publisher = users_cache_1.user_id
      AND users_cache_1.rn = 1

    LEFT JOIN (
        SELECT *,
          ROW_NUMBER() OVER (PARTITION BY worlds_cache.world_id ORDER BY id DESC) AS rn
        FROM worlds_cache
      ) worlds_cache_1
      ON worlds.id = worlds_cache_1.world_id
      AND worlds_cache_1.rn = 1

    LEFT JOIN tags_worlds
      ON worlds.id = tags_worlds.worlds_id

    LEFT JOIN tags
      ON tags_worlds.tags_id = tags.id

    LEFT JOIN (
      SELECT world_id, COUNT(*) AS self_visits
      FROM activities
      GROUP BY world_id
    ) activities_1
      ON worlds.id = activities_1.world_id

    WHERE
    ($1 IS NULL OR tags_worlds.tags_id = $1)
    AND ($2 IS NULL OR (CASE WHEN $2 THEN worlds.registered_at IS NOT NULL ELSE worlds.registered_at IS NULL END))
    AND ($3 IS NULL OR (CASE WHEN $3 THEN tags_worlds.tags_id IS NOT NULL ELSE tags_worlds.tags_id IS NULL END))

    ORDER BY worlds.registered_at DESC
    ;
    ",
    filter.tag_id,
    filter.registered,
    filter.classified,
  ).fetch_all(get_pool().await).await?;

  Ok(res.into_iter().map(|q: sql_return_types::World| {
    World {
      id: q.id,
      uuid: q.uuid,
      publisher: q.publisher,
      publisher_name: q.publisher_name,
      description: q.description,
      title: q.title,
      visits: q.visits,
      favorites: q.favorites,
      capacity: q.capacity,
      published_at: q.published_at,
      supports_windows: q.does_support_windows.map(|v| v != 0),
      supports_android: q.does_support_android.map(|v| v != 0),
      supports_ios: q.does_support_ios.map(|v| v != 0),
      self_visits: q.self_visits,
    }
  }).collect())
}

pub async fn update_registered(id: i64, is_registered: bool) -> Result<(), sqlx::Error> {
  let registered_at = if is_registered {
    Some(chrono::Utc::now().timestamp())
  } else {
    None
  };

  sqlx::query!(
    "UPDATE worlds SET registered_at = ? WHERE id = ?;",
    registered_at,
    id
  ).execute(get_pool().await).await?;

  Ok(())
}

pub async fn new_session(world_id: i64, started_at: i64, ended_at: i64) -> Result<(), sqlx::Error> {
  sqlx::query!(
    "INSERT INTO activities (world_id, started_at, ended_at) VALUES (
      ?,
      ?,
      ?
    );",
    world_id,
    started_at,
    ended_at
  ).execute(get_pool().await).await?;
  Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WorldQueryFilters {
  tag_id: Option<i64>,
  registered: Option<bool>,
  classified: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum SortBy {
  Recency,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct World {
  id: i64,

  uuid: String,

  #[serde(skip_serializing_if = "Option::is_none")]
  publisher: Option<i64>,

  #[serde(skip_serializing_if = "Option::is_none")]
  publisher_name: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  title: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  visits: Option<i64>,

  #[serde(skip_serializing_if = "Option::is_none")]
  favorites: Option<i64>,

  #[serde(skip_serializing_if = "Option::is_none")]
  capacity: Option<i64>,

  #[serde(skip_serializing_if = "Option::is_none")]
  published_at: Option<i64>,

  #[serde(skip_serializing_if = "Option::is_none")]
  supports_windows: Option<bool>,

  #[serde(skip_serializing_if = "Option::is_none")]
  supports_android: Option<bool>,

  #[serde(skip_serializing_if = "Option::is_none")]
  supports_ios: Option<bool>,

  #[serde(skip_serializing_if = "Option::is_none")]
  self_visits: Option<i64>,
}

mod sql_return_types {
  #[derive(sqlx::FromRow)]
  pub struct World {
    pub id: i64,
    pub uuid: String,
    pub publisher: Option<i64>,
    pub publisher_name: Option<String>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub visits: Option<i64>,
    pub favorites: Option<i64>,
    pub capacity: Option<i64>,
    pub published_at: Option<i64>,
    pub does_support_windows: Option<i64>, // is stored as i64 but actually bool
    pub does_support_android: Option<i64>, // is stored as i64 but actually bool
    pub does_support_ios: Option<i64>, // is stored as i64 but actually bool
    pub self_visits: Option<i64>,
  }
}
